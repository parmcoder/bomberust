/**
Normally, we would track an event by having a variable that is mutable and send signal to trigger event.
Likewise, this event system in amethyst will do that for you.
*/

#[derive(Debug)]
pub struct ResetFallTimerEvent {}
#[derive(Debug)]
pub struct PieceLandEvent {}
