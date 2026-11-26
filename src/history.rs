#[derive(Debug, Clone)]
pub struct CheckpointQuiz {
    #[allow(dead_code)]
    pub section: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub historical_explanation: &'static str,
    pub question: &'static str,
    pub options: [&'static str; 4],
    #[allow(dead_code)]
    pub correct_option: usize,
    pub did_you_know: &'static str,
}

pub fn get_checkpoint_quizzes() -> [CheckpointQuiz; 3] {
    [
        CheckpointQuiz {
            section: 1,
            title: "SECTION 1: THE PALACE GROUNDS",
            subtitle: "Rahewin's Chronicle & The Itinerant Kingship",
            historical_explanation: "The contemporary chronicler Rahewin described the Imperial Palace in Kaiserslautern as an imposing palace, surrounded by a strong defensive wall, a large fish-rich lake (Kaiserwoog), and a wildlife park with deer.",
            question: "Why did medieval emperors and kings travel between different palaces?",
            options: [
                "The empire had no fixed capital; rulers traveled to exercise presence & justice.",
                "They were searching for hidden Roman gold treasures in the forests.",
                "The palaces were abandoned every winter due to lack of heating.",
                "They were fleeing from frequent volcanic eruptions in the Palatinate Forest.",
            ],
            correct_option: 0,
            did_you_know: "The Holy Roman Empire was an 'Itinerant Kingship'. The emperor ruled wherever he and his entourage were present!",
        },
        CheckpointQuiz {
            section: 2,
            title: "SECTION 2: THE GREAT DOMUS",
            subtitle: "The Imperial Residential & Ceremonial Building",
            historical_explanation: "The Domus was the monumental main residential building of the Imperial Palace. Archaeological excavations confirm its massive dimensions: approximately 25 meters long, 19 meters wide, and 19 meters high, built from red sandstone.",
            question: "What was the 'Domus' of the Kaiserslautern Imperial Palace?",
            options: [
                "The central imperial residential and main palace building (approx. 25m x 19m x 19m).",
                "A purely military watchtower for archers.",
                "An underground prison dungeon for captured knights.",
                "The imperial horse stable outside the city walls.",
            ],
            correct_option: 0,
            did_you_know: "At 19 meters tall, the Domus towered far above the Lauter valley and symbolized the Staufer imperial power!",
        },
        CheckpointQuiz {
            section: 3,
            title: "SECTION 3: THE LATER DOUBLE CHAPEL",
            subtitle: "Construction Phases: From Barbarossa to Frederick II",
            historical_explanation: "The famous Double Chapel was only built in a later construction phase under Barbarossa's grandson Emperor Frederick II in the 13th century, and was not part of the original 1158 complex built by Barbarossa.",
            question: "Which ruler is associated with the construction of the later Double Chapel?",
            options: [
                "Emperor Frederick II (Barbarossa's grandson, later construction phase in the 13th century).",
                "Charlemagne in the 8th century.",
                "King Rudolf of Habsburg in the year 1276.",
                "Count Johann Casimir in the 16th century.",
            ],
            correct_option: 0,
            did_you_know: "In a Double Chapel, the emperor prayed on the upper floor while the entourage attended mass on the lower floor!",
        },
    ]
}
