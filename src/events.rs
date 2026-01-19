use crossterm::event::Event as CrosstermEvent;

pub enum Event {
    Input(CrosstermEvent),
    Refresh,
}
