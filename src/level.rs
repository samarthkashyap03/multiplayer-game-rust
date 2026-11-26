use macroquad::math::Rect;

use crate::constants::{
    CHECKPOINT_1_Y, CHECKPOINT_2_Y, CHECKPOINT_3_Y, WORLD_BOTTOM_Y, WORLD_WIDTH,
};
use crate::mechanisms::{CarryObject, RopeAnchor, SharedMechanism};

pub struct Level {
    pub platforms: Vec<Rect>,
    pub hazards: Vec<Rect>,
    pub ropes: Vec<RopeAnchor>,
    pub mechanisms: Vec<SharedMechanism>,
    pub carry_objects: Vec<CarryObject>,
}

impl Level {
    pub fn build_kaiserpfalz() -> Self {
        let mut platforms = Vec::new();
        let mut hazards = Vec::new();
        let mut ropes = Vec::new();
        let mut mechanisms = Vec::new();
        let mut carry_objects = Vec::new();

        // Outer Palace Walls (left & right borders)
        platforms.push(Rect::new(-50.0, 0.0, 50.0, WORLD_BOTTOM_Y + 100.0));
        platforms.push(Rect::new(WORLD_WIDTH, 0.0, 50.0, WORLD_BOTTOM_Y + 100.0));

        // Ground Floor (Section 1 Courtyard Base)
        // Left Courtyard bank
        platforms.push(Rect::new(0.0, WORLD_BOTTOM_Y - 50.0, 440.0, 70.0));
        // Right Courtyard bank
        platforms.push(Rect::new(660.0, WORLD_BOTTOM_Y - 50.0, WORLD_WIDTH - 660.0, 70.0));

        // ---------------------------------------------------------------------
        // SECTION 1: PALACE GROUNDS (Y: 3000 to 2050)
        // ---------------------------------------------------------------------
        // Hazard: Deep water pond gap between banks
        hazards.push(Rect::new(430.0, WORLD_BOTTOM_Y - 20.0, 240.0, 50.0));

        // Safe stepping stone bridge across pond
        // platforms.push(Rect::new(420.0, WORLD_BOTTOM_Y - 80.0, 260.0, 20.0));

        // Tier 1: Low Courtyard Terraces (y: 2850)
        platforms.push(Rect::new(180.0, 2850.0, 240.0, 20.0));
        platforms.push(Rect::new(680.0, 2850.0, 240.0, 20.0));

        // Tier 2: Central Terrace over fishpond (y: 2775)
        platforms.push(Rect::new(380.0, 2775.0, 340.0, 22.0));

        // Tier 3: Scaffolding climbing paths (y: 2700)
        platforms.push(Rect::new(120.0, 2700.0, 280.0, 20.0));
        platforms.push(Rect::new(680.0, 2700.0, 280.0, 20.0));

        // Tier 4: Mid walkways (y: 2620)
        platforms.push(Rect::new(260.0, 2620.0, 260.0, 20.0));
        platforms.push(Rect::new(560.0, 2620.0, 260.0, 20.0));

        // Tier 5: Central approach terrace (y: 2540)
        platforms.push(Rect::new(380.0, 2540.0, 340.0, 22.0));

        // Tier 6: High terrace with Rope Anchor 1 (y: 2460)
        platforms.push(Rect::new(450.0, 2460.0, 200.0, 24.0));
        // Rope Anchor 1 (Player can drop rope to help teammate up)
        ropes.push(RopeAnchor::new(0, 550.0, 2460.0, 320.0));

        // Tier 7: Lower approach to Gate 1 (y: 2375)
        platforms.push(Rect::new(160.0, 2375.0, 260.0, 20.0));
        platforms.push(Rect::new(660.0, 2375.0, 260.0, 20.0));

        // Tier 8: Mid approach to Gate 1 (y: 2290)
        platforms.push(Rect::new(280.0, 2290.0, 240.0, 20.0));
        platforms.push(Rect::new(560.0, 2290.0, 240.0, 20.0));

        // Tier 9: Steps leading up to Gate 1 entrance (y: 2205)
        platforms.push(Rect::new(180.0, 2205.0, 240.0, 20.0));
        platforms.push(Rect::new(660.0, 2205.0, 240.0, 20.0));

        // Checkpoint 1 Entrance floor
        platforms.push(Rect::new(0.0, CHECKPOINT_1_Y + 70.0, 480.0, 25.0));
        platforms.push(Rect::new(620.0, CHECKPOINT_1_Y + 70.0, WORLD_WIDTH - 620.0, 25.0));

        // Shared Mechanism 1: Dual levers to open Portcullis Gate 1 into Domus
        // Switch A on left ledge, Switch B on right ledge
        mechanisms.push(SharedMechanism::new(
            0,
            (250.0, CHECKPOINT_1_Y + 65.0),
            (850.0, CHECKPOINT_1_Y + 65.0),
            Rect::new(480.0, CHECKPOINT_1_Y - 30.0, 140.0, 120.0),
            CHECKPOINT_1_Y - 140.0,
        ));

        // Historical Markers in Section 1 (Wall, Fishpond, Wildlife Park)
        carry_objects.push(CarryObject::new(
            0,
            "1. Defensive Ring Wall",
            1,
            160.0,
            2664.0,
            470.0,
            CHECKPOINT_1_Y + 45.0,
        ));
        carry_objects.push(CarryObject::new(
            1,
            "2. Kaiserwoog Pond",
            1,
            860.0,
            2664.0,
            530.0,
            CHECKPOINT_1_Y + 45.0,
        ));
        carry_objects.push(CarryObject::new(
            2,
            "3. Staufer Deer Park",
            1,
            460.0,
            2739.0,
            590.0,
            CHECKPOINT_1_Y + 45.0,
        ));

        // ---------------------------------------------------------------------
        // SECTION 2: THE DOMUS (Y: 2050 to 1050)
        // ---------------------------------------------------------------------
        // Threshold stepping platform past Gate 1 into Domus
        platforms.push(Rect::new(450.0, 2030.0, 200.0, 20.0));
        // The Great Imperial Residential Palace (25m x 19m x 19m)
        platforms.push(Rect::new(200.0, 1940.0, 220.0, 22.0));
        platforms.push(Rect::new(680.0, 1940.0, 220.0, 22.0));

        platforms.push(Rect::new(420.0, 1820.0, 260.0, 22.0));

        platforms.push(Rect::new(120.0, 1700.0, 200.0, 20.0));
        platforms.push(Rect::new(780.0, 1700.0, 200.0, 20.0));

        // Winch platform & high ledge with Rope Anchor 2
        platforms.push(Rect::new(340.0, 1580.0, 420.0, 24.0));
        ropes.push(RopeAnchor::new(1, 400.0, 1580.0, 220.0));

        platforms.push(Rect::new(160.0, 1450.0, 240.0, 20.0));
        platforms.push(Rect::new(700.0, 1450.0, 240.0, 20.0));

        platforms.push(Rect::new(400.0, 1340.0, 300.0, 22.0));
        platforms.push(Rect::new(200.0, 1220.0, 240.0, 20.0));
        platforms.push(Rect::new(660.0, 1220.0, 240.0, 20.0));

        // Checkpoint 2 Entrance floor
        platforms.push(Rect::new(0.0, CHECKPOINT_2_Y + 70.0, 480.0, 25.0));
        platforms.push(Rect::new(620.0, CHECKPOINT_2_Y + 70.0, WORLD_WIDTH - 620.0, 25.0));

        // Shared Mechanism 2: Domus Great Hall Gate
        mechanisms.push(SharedMechanism::new(
            1,
            (260.0, CHECKPOINT_2_Y + 65.0),
            (840.0, CHECKPOINT_2_Y + 65.0),
            Rect::new(480.0, CHECKPOINT_2_Y - 30.0, 140.0, 120.0),
            CHECKPOINT_2_Y - 140.0,
        ));

        // Stone Foundation Blocks to position in Domus
        carry_objects.push(CarryObject::new(
            3,
            "Domus Block 25m North",
            2,
            140.0,
            1670.0,
            490.0,
            CHECKPOINT_2_Y + 45.0,
        ));
        carry_objects.push(CarryObject::new(
            4,
            "Domus Block 19m South",
            2,
            800.0,
            1670.0,
            570.0,
            CHECKPOINT_2_Y + 45.0,
        ));

        // ---------------------------------------------------------------------
        // SECTION 3: THE LATER PALACE CHAPEL (Y: 1050 to 180)
        // ---------------------------------------------------------------------
        // Double Chapel under Frederick II (13th Century)
        platforms.push(Rect::new(240.0, 940.0, 200.0, 22.0));
        platforms.push(Rect::new(660.0, 940.0, 200.0, 22.0));

        platforms.push(Rect::new(420.0, 830.0, 260.0, 22.0));

        platforms.push(Rect::new(140.0, 710.0, 220.0, 20.0));
        platforms.push(Rect::new(740.0, 710.0, 220.0, 20.0));

        // Chapel balcony & Rope Anchor 3
        platforms.push(Rect::new(360.0, 590.0, 380.0, 24.0));
        ropes.push(RopeAnchor::new(2, 680.0, 590.0, 220.0));

        platforms.push(Rect::new(160.0, 470.0, 240.0, 20.0));
        platforms.push(Rect::new(700.0, 470.0, 240.0, 20.0));

        platforms.push(Rect::new(380.0, 350.0, 340.0, 22.0));

        // Summit of Kaiserpfalz: Royal Balcony & Observation Deck
        platforms.push(Rect::new(0.0, CHECKPOINT_3_Y + 70.0, 480.0, 25.0));
        platforms.push(Rect::new(620.0, CHECKPOINT_3_Y + 70.0, WORLD_WIDTH - 620.0, 25.0));

        // Shared Mechanism 3: Royal Bell Altar to trigger final victory
        mechanisms.push(SharedMechanism::new(
            2,
            (250.0, CHECKPOINT_3_Y + 65.0),
            (850.0, CHECKPOINT_3_Y + 65.0),
            Rect::new(480.0, CHECKPOINT_3_Y - 30.0, 140.0, 120.0),
            CHECKPOINT_3_Y - 140.0,
        ));

        // Architectural Fragments (Chapel vs Domus building phases)
        carry_objects.push(CarryObject::new(
            5,
            "Chapel Round Arch (13th c.)",
            3,
            160.0,
            680.0,
            500.0,
            CHECKPOINT_3_Y + 45.0,
        ));
        carry_objects.push(CarryObject::new(
            6,
            "Staufer Column Frederick II",
            3,
            760.0,
            680.0,
            580.0,
            CHECKPOINT_3_Y + 45.0,
        ));

        Self {
            platforms,
            hazards,
            ropes,
            mechanisms,
            carry_objects,
        }
    }
}
