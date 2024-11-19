use super::{Bar, BarExt};
use std::{collections::HashSet, io::Result};

/// RowManager allows to store and update many progress bars.
///
/// `nrows` is the number of progress bars to display at once.
/// All other bars are hidden and visible once any active progress bar is completed.
/// Traces of progress are left in terminal if `leave` is `true` else progress bar is cleared.
///
/// # Note
///
/// Cursor positions are not restored by RowManager.
///
/// # Example
///
/// ```
/// use kdam::{tqdm, BarExt, RowManager};
///
/// let mut manager = RowManager::new(3);
/// let pb_index = manager.push(tqdm!(total = 100)).unwrap();
///
/// for _ in 0..100 {
///     manager.get_mut(pb_index).unwrap().update(1).unwrap();
///     manager.notify(pb_index).unwrap();
/// }
///
/// manager.remove(pb_index);
/// ```
pub struct RowManager {
    acquired_pos: HashSet<u16>,
    avaliable_pos: HashSet<u16>,
    bars: Vec<(Bar, bool)>,
    nrows: u16,
}

impl RowManager {
    // -----------------------------------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------------------------------

    /// Create a new [RowManager](crate::RowManager) with specified number of rows.
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::RowManager;
    ///
    /// // Display 3 progress bars at once.
    /// let manager = RowManager::new(3);
    /// ```
    pub fn new(nrows: u16) -> Self {
        Self {
            acquired_pos: HashSet::new(),
            avaliable_pos: HashSet::new(),
            bars: vec![],
            nrows,
        }
    }

    /// Create a new [RowManager](crate::RowManager) from terminal window size.
    ///
    /// # Example
    ///
    /// ```
    /// use kdam::RowManager;
    ///
    /// let mut manager = RowManager::from_window_size();
    /// ```
    pub fn from_window_size() -> Self {
        Self {
            acquired_pos: HashSet::new(),
            avaliable_pos: HashSet::new(),
            bars: vec![],
            nrows: terminal_size::terminal_size()
                .map(|(_, h)| h.0)
                .unwrap_or(3)
                - 2,
        }
    }

    // -----------------------------------------------------------------------------------------
    // Methods
    // -----------------------------------------------------------------------------------------

    /// Push a progress bar returning back it's index.
    pub fn push(&mut self, mut pb: Bar) -> Result<usize> {
        pb.position = self.acquired_pos.len() as u16;
        let disable = pb.disable;

        if self.nrows > pb.position {
            pb.refresh()?;
            self.acquired_pos.insert(pb.position);
        } else {
            pb.disable = true;
        }

        self.bars.push((pb, disable));
        Ok(self.bars.len() - 1)
    }

    /// Returns a mutable reference to progress bar.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Bar> {
        self.bars.get_mut(index).map(|(x, _)| x)
    }

    /// Update and print the required stuff for progress bar at that index.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    pub fn notify(&mut self, index: usize) -> Result<()> {
        let remaining_bars = self.bars.len()
            - self
                .bars
                .iter()
                .map(|(x, _)| x.completed())
                .filter(|x| *x)
                .count();

        let (pb, disable) = self.bars.get_mut(index).unwrap();

        if !*disable && pb.completed() {
            if pb.leave {
                let text = pb.render();
                pb.writer.print(format!("\r{}\n", text).as_bytes())?;
            }

            pb.clear()?;
            pb.disable = true;

            if self.acquired_pos.remove(&pb.position) {
                self.avaliable_pos.insert(pb.position);
            }
        }

        if self.nrows as usize > remaining_bars {
            let mut count = 0;

            for (bar, disable) in self.bars.iter_mut() {
                if !*disable && !bar.completed() {
                    if bar.position != count {
                        bar.disable = false;
                        bar.clear()?;
                        bar.position = count;
                        bar.refresh()?;
                    }

                    count += 1;
                }
            }
        } else {
            pb.writer.print_at(
                self.acquired_pos.iter().max().unwrap_or(&0) + 1,
                if self.nrows as usize == remaining_bars {
                    "                      ".as_bytes()
                } else {
                    " ... (more hidden) ...".as_bytes()
                },
            )?;

            for (bar, disable) in self.bars.iter_mut() {
                if !*disable && !bar.completed() {
                    if let Some(pos) = self.avaliable_pos.iter().min() {
                        if bar.disable && bar.position != *pos {
                            bar.position = *pos;

                            if self.nrows > bar.position {
                                bar.disable = false;
                            }

                            bar.refresh()?;

                            if self.avaliable_pos.remove(&bar.position) {
                                self.acquired_pos.insert(bar.position);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Removes a progress bar and returns it.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    pub fn remove(&mut self, index: usize) -> Bar {
        let (pb, _) = self.bars.remove(index);

        if self.acquired_pos.remove(&pb.position) {
            self.avaliable_pos.insert(pb.position);
        }

        pb
    }
}
