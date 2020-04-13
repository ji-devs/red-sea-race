use super::player::TweenEnding;
use super::timeline::TweenTimeline;

#[derive(Debug)]
pub enum TweenEvent {
    StartByName(&'static str, TweenEnding),
    Start(TweenTimeline, TweenEnding),
    Stop
}