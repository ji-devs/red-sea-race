mod targets;
mod player;
mod timeline;
mod lookup;
mod events;
#[cfg(test)]
mod tests;

pub use targets::*;
pub use player::*;
pub use timeline::*;
pub use lookup::*;
pub use events::*;


#[derive(Debug, Clone)]
pub enum Tween{
    Translation(Vec3Tween),
    Rotation(ScalarTween),
    Scale(Vec3Tween),
    ColorAdjust(ColorTween),
}
impl Tween {
    pub fn duration(&self) -> f64 {
        self.info().duration
    }

    pub fn info(&self) -> &TweenInfo {
        match self {
            Self::Translation(tween) => &tween.info,
            Self::Scale(tween) => &tween.info,
            Self::Rotation(tween) => &tween.info,
            Self::ColorAdjust(tween) => &tween.info,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TweenInfo {
    pub entity: shipyard::prelude::EntityId,
    pub duration: f64,
    pub easing: Option<f64>,
}

/*
    The tween methodology is pretty much just to support our dragonbones needs
    though it could definitely be expanded!

    Overall the concept is that tween data is lightweight and easily cloned
    It gets instantiated per-instance

    Would be nice to instead make it reference-based, but that's tricky
    Since each tween targets a specific entity (bone, texture, etc.)
*/


/*
//Animations are simply tweens that have prebaked values in a lookup


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
    Switch(TweenTrack, Box<TweenEnding>),
    SwitchByName(&'static str, Box<TweenEnding>)
}
*/

/*
//Just a lookup to get tween sequences by name
#[derive(Deref, DerefMut, Debug)]
pub struct TweensByName(pub HashMap<String, TweenTrack>);

//To play a given Tween sequence
#[derive(Debug)]
pub struct TweenPlayer {
    pub track: TweenTrack, 
    pub index: usize,
    pub playhead: f64,
    pub state: TweenState,
    pub ending: TweenEnding,
    pub duration: f64,
}


impl TweenPlayer {
    pub fn new(track:TweenTrack, ending: TweenEnding) -> Self {
        let duration = track.duration();
        Self {
            track,
            index: 0,
            playhead: 0.0,
            state: TweenState::Playing,
            ending,
            duration 
        }
    }

}



//To play a given Tween sequence
#[derive(Debug, Clone)]
pub enum TweenTrack {
    Tween(Tween),
    Sequence(Vec<Tween>),
    Group(Vec<Tween>),
    Track(Box<TweenTrack>),
    TrackSequence(Box<Vec<TweenTrack>>),
    TrackGroup(Box<Vec<TweenTrack>>),
}

impl TweenTrack {
    pub fn duration(&self) -> f64 {
        match self {
            Self::Tween(tween) => tween.duration,
            Self::Sequence(tweens) => tweens.iter().fold(0.0, |total, tween| total + tween.duration),
            Self::Group(tweens) => tweens.iter().fold(0.0, |max, tween| {
                let duration = tween.duration;
                if duration > max {
                    duration
                } else {
                    max
                }
            }),
            Self::Track(track) => track.duration(),
            Self::TrackSequence(tracks) => tracks.iter().fold(0.0, |total, track| total + track.duration()),
            Self::TrackGroup(tracks) => tracks.iter().fold(0.0, |max, track| {
                let duration = track.duration();
                if duration > max {
                    duration
                } else {
                    max
                }
            }),
        }
    }

    //returns the current target(s) and normalized time(s)
    //TODO - write tests :P
    pub fn get_tween_modifier(&self, playhead: f64) -> Option<TweenModifier> {
        match self {
            Self::Tween(tween) => {
                if playhead < tween.duration {
                    let n_time = playhead / tween.duration;
                    Some(TweenModifier::Single(n_time, tween))
                } else {
                    None
                }
            },
            Self::Sequence(tweens) => {
                let mut start_time = 0.0;
                let mut end_time = 0.0;

                for tween in tweens {
                    end_time += tween.duration;
                    if playhead > start_time && playhead < end_time {
                        let n_time = (playhead - start_time) / tween.duration;
                        return Some(TweenModifier::Single(n_time, tween))
                    }

                    start_time += tween.duration;
                }
                None
            }
            Self::Group(tweens) => {
                if playhead < self.duration() {
                    let group = tweens.iter().map(|tween| {
                        let n_time = playhead / tween.duration;
                        (n_time, tween)  
                    })
                    .collect();

                    Some(TweenModifier::Group(group))
                } else {
                    None
                }
            }
            Self::Track(track) => track.get_tween_modifier(playhead),
            Self::TrackSequence(tracks) => {
                let mut start_time = 0.0;
                let mut end_time = 0.0;
                for track in tracks.iter() {
                    let duration = track.duration();
                    end_time += duration;
                    if playhead > start_time && playhead < end_time {
                        return track.get_tween_modifier(playhead - start_time);
                    }

                    start_time += duration;
                }
                None
            }, 
            Self::TrackGroup(tracks) => {
                if playhead < self.duration() {
                    let modifiers:Vec<(f64, &'_ Tween)> = tracks.iter().map(|track| {
                        track.get_tween_modifier(playhead)
                    })
                    .filter(|m| m.is_some())
                    .map(|m| m.unwrap_throw()) 
                    .flat_map(|m| match m {
                        TweenModifier::Single(n_time, tween) => vec![(n_time, tween)],
                        TweenModifier::Group(modifiers) => modifiers
                    })
                    .collect();


                    Some(TweenModifier::Group(modifiers))
                } else {
                    None
                }
            }

            _ => None 
        }
    }
}

//enum to contain get_tween_modifier
pub enum TweenModifier<'a> {
    Single(f64, &'a Tween),
    Group(Vec<(f64, &'a Tween)>),
}

#[derive(Debug, Clone)]
pub struct Tween {
    pub entity: EntityId,
    pub target: TweenTarget,
    pub duration: f64,
    pub easing: Option<f64>
}

//Animations are simply tweens that have prebaked values in a lookup
#[derive(Debug)]
pub enum TweenEvent {
    //The name of the animation sequence, and ending style
    StartByName(&'static str, TweenEnding),
    Start(TweenTrack, TweenEnding),
    Stop
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
    Switch(TweenTrack, Box<TweenEnding>),
    SwitchByName(&'static str, Box<TweenEnding>)
}


#[derive(Debug, Clone)]
pub enum TweenTarget {
    Translation(TranslationTweenTarget),
    Rotation(RotationTweenTarget),
    Color(ColorTweenTarget),
}

#[derive(Debug, Clone)]
pub struct TranslationTweenTarget {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct RotationTweenTarget {
    pub rotation: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ColorTweenTarget {
    pub alpha_overlay: Option<f32>,
    pub red_overlay: Option<f32>,
    pub green_overlay: Option<f32>,
    pub blue_overlay: Option<f32>,

    pub alpha_offset: Option<f32>,
    pub red_offset: Option<f32>,
    pub green_offset: Option<f32>,
    pub blue_offset: Option<f32>
}
*/