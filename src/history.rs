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
            title: "ABSCHNITT 1: DIE PALASTANLAGE (PALACE GROUNDS)",
            subtitle: "Rahewin's Chronik & Das Reisekönigtum",
            historical_explanation: "Der zeitgenössische Chronist Rahewin beschrieb die Kaiserpfalz in Kaiserslautern als imposanten Palast, umgeben von einer starken Wehrmauer, einem großen fischreichen See (Kaiserwoog) und einem Wildpark mit Hirschen.",
            question: "Warum reisten mittelalterliche Kaiser und Könige zwischen verschiedenen Pfalzen umher?",
            options: [
                "Das Reich hatte keine feste Hauptstadt; Herrscher reisten, um Präsenz & Recht auszuüben.",
                "Sie suchten nach verborgenen römischen Goldschätzen in den Wäldern.",
                "Die Pfalzen wurden jeden Winter wegen fehlender Heizung aufgegeben.",
                "Sie flüchteten vor häufigen Vulkanausbrüchen im Pfälzerwald.",
            ],
            correct_option: 0,
            did_you_know: "Das Heilige Römische Reich war ein 'Reisekönigtum' (Itinerant Kingship). Der Kaiser regierte dort, wo er sich mit seinem Gefolge aufhielt!",
        },
        CheckpointQuiz {
            section: 2,
            title: "ABSCHNITT 2: DER GROSSE DOMUS",
            subtitle: "Das kaiserliche Wohn- und Repräsentationsgebäude",
            historical_explanation: "Der Domus war das monumentale Hauptwohngebäude der Kaiserpfalz. Archäologische Grabungen belegen seine gewaltigen Maße: rund 25 Meter Länge, 19 Meter Breite und 19 Meter Höhe aus rotem Buntsandstein.",
            question: "Was war der 'Domus' der Kaiserslauterer Kaiserpfalz?",
            options: [
                "Das zentrale kaiserliche Wohn- und Hauptpalastgebäude (ca. 25m x 19m x 19m).",
                "Ein reiner militärischer Wachturm für Bogenschützen.",
                "Ein unterirdisches Gefängnisverlies für gefangene Ritter.",
                "Der kaiserliche Pferdestall außerhalb der Stadtmauern.",
            ],
            correct_option: 0,
            did_you_know: "Mit 19 Metern Höhe ragte der Domus weit über das Tal der Lauter hinaus und symbolisierte die staufische Reichsgewalt!",
        },
        CheckpointQuiz {
            section: 3,
            title: "ABSCHNITT 3: DIE SPÄTERE DOPPELKAPELLE",
            subtitle: "Bauphasen: Von Barbarossa zu Friedrich II.",
            historical_explanation: "Die berühmte Doppelkapelle entstand erst in einer späteren Bauphase unter Barbarossas Enkel Kaiser Friedrich II. im 13. Jahrhundert und war nicht Teil der ursprünglichen 1158er Anlage Barbarossas.",
            question: "Welcher Herrscher wird mit dem Bau der späteren Doppelkapelle in Verbindung gebracht?",
            options: [
                "Kaiser Friedrich II. (Enkel Barbarossas, spätere Bauphase im 13. Jahrhundert).",
                "Karl der Große im 8. Jahrhundert.",
                "König Rudolf von Habsburg im Jahr 1276.",
                "Graf Johann Casimir im 16. Jahrhundert.",
            ],
            correct_option: 0,
            did_you_know: "In einer Doppelkapelle betete der Kaiser im oberen Stockwerk, während das Gefolge die Messe im unteren Geschoss verfolgte!",
        },
    ]
}
