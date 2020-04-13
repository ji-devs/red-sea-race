use super::{Tween};
use itertools::Itertools;

//To play a given Tween sequence
#[derive(Debug, Clone)]
pub enum TweenTimeline {
    Clip(Tween),
    Sequence(Box<Vec<TweenTimeline>>),
    Group(Box<Vec<TweenTimeline>>),
}

impl TweenTimeline {
    pub fn duration(&self) -> f64 {
        match self {
            Self::Clip(tween) => tween.duration(),
            Self::Sequence(timelines) => timelines.iter().fold(0.0, |total, timeline| total + timeline.duration()),

            Self::Group(timelines) => timelines.iter().fold(0.0, |max, timeline| {
                let duration = timeline.duration();
                if duration > max {
                    duration
                } else {
                    max
                }
            }),
        }
    }
    pub fn len(&self) -> usize {
        match self {
            Self::Clip(_) => 1,
            Self::Sequence(timelines) => timelines.len(),
            Self::Group(timelines) => timelines.len(),
        }
    }

    pub fn as_clip(&self) -> Option<&Tween> {
        if let Self::Clip(tween) = self {
            Some(tween)
        } else {
            None
        }
    }

    pub fn as_group(&self) -> Option<&Box<Vec<TweenTimeline>>> {
        if let Self::Group(timelines) = self {
            Some(timelines)
        } else {
            None
        }
    }
    pub fn as_sequence(&self) -> Option<&Box<Vec<TweenTimeline>>> {
        if let Self::Sequence(timelines) = self {
            Some(timelines)
        } else {
            None
        }
    }
   
   
    //TODO - this could get called a lot, would be nicer to return an iterator
    //iterator could use a single owned Vec (maybe even with the capacity of greatest depth)
    //also, think about ultimately getting the perc - e.g. passing down the playhead - start_time
    pub fn get_active_tweens(&self, playhead:f64) -> Option<Vec<(f64, &Tween)>> {
        let self_duration = self.duration();

        if playhead > self_duration {
            return None;
        }

        let mut tweens:Vec<(f64, &Tween)> = Vec::new();


        match self {
            Self::Clip(tween) => {
                let progress = playhead / self_duration;
                tweens.push((progress, tween));
            },

            Self::Group(timelines) => {
                for timeline in timelines.iter() {
                    if let Some(active_tweens) = timeline.get_active_tweens(playhead) {
                        for tween in active_tweens {
                            tweens.push(tween);
                        }
                    }
                }
            },

            Self::Sequence(timelines) => {
                let mut start_time = 0.0;
                let mut end_time = 0.0;
                for timeline in timelines.iter() {
                    let timeline_duration = timeline.duration();
                    end_time += timeline_duration;

                    if playhead > start_time && playhead < end_time {
                        if let Some(active_tweens) = timeline.get_active_tweens(playhead - start_time) {
                            for tween in active_tweens {
                                tweens.push(tween);
                            }
                        }
                    }

                    start_time += timeline_duration;
                }
            },
        }

        if tweens.len() != 0 {
            Some(tweens)
        } else {
            None
        }
    }
}