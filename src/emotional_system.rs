use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EmotionType {
    Distress,
    Fear,
    Hope,
    Joy,
    Satisfaction,
    FearConfirmed,
    Disappointment,
    Relief,
    HappyFor,
    Resentment,
    Pity,
    Gloating,
    Gratitude,
    Anger,
    Gratification,
    Remorse,
}

impl EmotionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmotionType::Distress => "distress",
            EmotionType::Fear => "fear",
            EmotionType::Hope => "hope",
            EmotionType::Joy => "joy",
            EmotionType::Satisfaction => "satisfaction",
            EmotionType::FearConfirmed => "fear-confirmed",
            EmotionType::Disappointment => "disappointment",
            EmotionType::Relief => "relief",
            EmotionType::HappyFor => "happy-for",
            EmotionType::Resentment => "resentment",
            EmotionType::Pity => "pity",
            EmotionType::Gloating => "gloating",
            EmotionType::Gratitude => "gratitude",
            EmotionType::Anger => "anger",
            EmotionType::Gratification => "gratification",
            EmotionType::Remorse => "remorse",
        }
    }

    pub fn pad_values(&self) -> [f64; 3] {
        match self {
            EmotionType::Distress => [-0.61, 0.28, -0.36],
            EmotionType::Fear => [-0.64, 0.6, -0.43],
            EmotionType::Hope => [0.51, 0.23, 0.14],
            EmotionType::Joy => [0.76, 0.48, 0.35],
            EmotionType::Satisfaction => [0.87, 0.2, 0.62],
            EmotionType::FearConfirmed => [-0.61, 0.06, -0.32],
            EmotionType::Disappointment => [-0.61, -0.15, -0.29],
            EmotionType::Relief => [0.29, -0.19, -0.28],
            EmotionType::HappyFor => [0.64, 0.35, 0.25],
            EmotionType::Resentment => [-0.35, 0.35, 0.29],
            EmotionType::Pity => [-0.52, 0.02, -0.21],
            EmotionType::Gloating => [-0.45, 0.48, 0.42],
            EmotionType::Gratitude => [0.64, 0.16, -0.21],
            EmotionType::Anger => [-0.51, 0.59, 0.25],
            EmotionType::Gratification => [0.69, 0.57, 0.63],
            EmotionType::Remorse => [-0.57, 0.28, -0.34],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub name: String,
    pub utility: f64,      // -1.0 to 1.0 (desire)
    pub likelihood: f64,   // 0.0 to 1.0
    pub is_maintenance: bool,
}

impl Goal {
    pub fn new(name: String, utility: f64, is_maintenance: bool) -> Self {
        Self {
            name,
            utility,
            likelihood: 0.5,
            is_maintenance,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emotion {
    pub emotion_type: EmotionType,
    pub intensity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmotionalState {
    pub emotions: Vec<Emotion>,
    pub goals: HashMap<String, Goal>,
    pub gain: f64,
}

impl EmotionalState {
    pub fn new() -> Self {
        Self {
            emotions: Vec::new(),
            goals: HashMap::new(),
            gain: 1.0,
        }
    }

    pub fn add_goal(&mut self, goal: Goal) {
        self.goals.insert(goal.name.clone(), goal);
    }

    pub fn update_emotional_state(&mut self, new_emotion: Emotion) {
        for emotion in &mut self.emotions {
            if emotion.emotion_type == new_emotion.emotion_type {
                emotion.intensity += new_emotion.intensity;
                return;
            }
        }
        self.emotions.push(new_emotion);
    }

    pub fn get_pad(&self) -> [f64; 3] {
        let mut p = 0.0;
        let mut a = 0.0;
        let mut d = 0.0;

        for e in &self.emotions {
            let pad = e.emotion_type.pad_values();
            p += e.intensity * pad[0];
            a += e.intensity * pad[1];
            d += e.intensity * pad[2];
        }

        let p_final = if p >= 0.0 { self.gain * p / (self.gain * p + 1.0) } else { -self.gain * p / (self.gain * p - 1.0) };
        let a_final = if a >= 0.0 { self.gain * a / (self.gain * a + 1.0) } else { -self.gain * a / (self.gain * a - 1.0) };
        let d_final = if d >= 0.0 { self.gain * d / (self.gain * d + 1.0) } else { -self.gain * d / (self.gain * d - 1.0) };

        [p_final, a_final, d_final]
    }

    pub fn appraise(&mut self, belief: &Belief) {
        let mut updates = Vec::new();

        for (i, goal_name) in belief.affected_goal_names.iter().enumerate() {
            if let Some(goal) = self.goals.get_mut(goal_name) {
                let congruence = belief.goal_congruences[i];
                let utility = goal.utility;
                
                let delta_likelihood = Self::static_calculate_delta_likelihood(goal, congruence, belief.likelihood, belief.is_incremental);
                
                updates.push((utility, delta_likelihood, goal.likelihood));
            }
        }

        for (utility, delta_likelihood, likelihood) in updates {
            self.evaluate_internal_emotion(utility, delta_likelihood, likelihood);
        }
    }

    fn static_calculate_delta_likelihood(goal: &mut Goal, congruence: f64, likelihood: f64, is_incremental: bool) -> f64 {
        let old_likelihood = goal.likelihood;
        if !goal.is_maintenance && (old_likelihood >= 1.0 || old_likelihood <= -1.0) {
            return 0.0;
        }

        let new_likelihood = if is_incremental {
            let next = old_likelihood + likelihood * congruence;
            next.max(-1.0).min(1.0)
        } else {
            (congruence * likelihood + 1.0) / 2.0
        };

        goal.likelihood = new_likelihood;
        new_likelihood - old_likelihood
    }

    fn evaluate_internal_emotion(&mut self, utility: f64, delta_likelihood: f64, likelihood: f64) {
        let positive = if utility >= 0.0 {
            delta_likelihood >= 0.0
        } else {
            delta_likelihood < 0.0
        };

        let mut emotion_types = Vec::new();

        if likelihood > 0.0 && likelihood < 1.0 {
            if positive {
                emotion_types.push(EmotionType::Hope);
            } else {
                emotion_types.push(EmotionType::Fear);
            }
        } else if (likelihood - 1.0).abs() < f64::EPSILON {
            if utility >= 0.0 {
                if delta_likelihood < 0.5 {
                    emotion_types.push(EmotionType::Satisfaction);
                }
                emotion_types.push(EmotionType::Joy);
            } else {
                if delta_likelihood < 0.5 {
                    emotion_types.push(EmotionType::FearConfirmed);
                }
                emotion_types.push(EmotionType::Distress);
            }
        } else if likelihood.abs() < f64::EPSILON {
            if utility >= 0.0 {
                if delta_likelihood > 0.5 {
                    emotion_types.push(EmotionType::Disappointment);
                }
                emotion_types.push(EmotionType::Distress);
            } else {
                if delta_likelihood > 0.5 {
                    emotion_types.push(EmotionType::Relief);
                }
                emotion_types.push(EmotionType::Joy);
            }
        }

        let intensity = (utility * delta_likelihood).abs();
        if intensity > 0.0 {
            for et in emotion_types {
                self.update_emotional_state(Emotion {
                    emotion_type: et,
                    intensity,
                });
            }
        }
    }
    
    pub fn decay(&mut self, decay_factor: f64) {
        self.emotions.retain_mut(|e| {
            e.intensity *= decay_factor;
            e.intensity > 0.001 // Threshold for removal
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    pub likelihood: f64,
    pub causal_agent_name: Option<String>,
    pub affected_goal_names: Vec<String>,
    pub goal_congruences: Vec<f64>,
    pub is_incremental: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appraisal_joy() {
        let mut state = EmotionalState::new();
        state.add_goal(Goal::new("Test Goal".to_string(), 1.0, false));
        
        // Event that fully confirms the goal
        let belief = Belief {
            likelihood: 1.0,
            causal_agent_name: None,
            affected_goal_names: vec!["Test Goal".to_string()],
            goal_congruences: vec![1.0],
            is_incremental: false,
        };
        
        state.appraise(&belief);
        
        assert!(state.emotions.iter().any(|e| e.emotion_type == EmotionType::Joy));
        let pad = state.get_pad();
        assert!(pad[0] > 0.0); // Pleasure should be positive
    }

    #[test]
    fn test_appraisal_hope_fear() {
        let mut state = EmotionalState::new();
        state.add_goal(Goal::new("Test Goal".to_string(), 1.0, false));
        
        // Event that increases likelihood but doesn't confirm
        let belief = Belief {
            likelihood: 0.1,
            causal_agent_name: None,
            affected_goal_names: vec!["Test Goal".to_string()],
            goal_congruences: vec![1.0],
            is_incremental: true,
        };
        
        state.appraise(&belief);
        
        assert!(state.emotions.iter().any(|e| e.emotion_type == EmotionType::Hope));
        
        // Event that decreases likelihood
        let belief2 = Belief {
            likelihood: 0.2,
            causal_agent_name: None,
            affected_goal_names: vec!["Test Goal".to_string()],
            goal_congruences: vec![-1.0],
            is_incremental: true,
        };
        
        state.appraise(&belief2);
        assert!(state.emotions.iter().any(|e| e.emotion_type == EmotionType::Fear));
    }
}
