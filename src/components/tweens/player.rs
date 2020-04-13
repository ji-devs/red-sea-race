use super::timeline::TweenTimeline;

//To play a given Tween sequence
#[derive(Debug)]
pub struct TweenPlayer {
    pub timeline: TweenTimeline, 
    pub index: usize,
    pub playhead: f64,
    pub state: TweenState,
    pub ending: TweenEnding,
    pub duration: f64,
}


impl TweenPlayer {
    pub fn new(timeline:TweenTimeline, ending: TweenEnding) -> Self {
        let duration = timeline.duration();
        Self {
            timeline,
            index: 0,
            playhead: 0.0,
            state: TweenState::Playing,
            ending,
            duration 
        }
    }
}

#[derive(Debug)]
pub enum TweenState {
    Playing,
    Paused,
    Finished
}


#[derive(Debug, Clone)]
pub enum TweenEnding {
    Loop,
    Remove,
    Switch(TweenTimeline, Box<TweenEnding>),
    SwitchByName(&'static str, Box<TweenEnding>)
}