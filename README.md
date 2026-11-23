# KaiserClimb: Die Kaiserpfalz Kaiserslautern
## 2D Cooperative Multiplayer Climbing Adventure (1–4 Players)

**KaiserClimb** is a cooperative 2D multiplayer platform climbing game inspired by the historical **Kaiserpfalz (Imperial Palace)** in Kaiserslautern, Germany.

Built in **Rust** using **Macroquad** for 2D rendering and physics, and WebSocket networking with an authoritative relay model allowing up to 4 players to control characters via their smartphones or PC keyboard.

---

## 🏰 Game Concept & Historical Sections

The game is structured as one continuous vertical climbing journey across three historical sections of the Kaiserpfalz:

1. **Section 1: The Palace Grounds (Die Palastanlage)**
   - **Environment**: Red sandstone boundary walls, medieval courtyard, Kaiserwoog fishpond, and wooden scaffolding.
   - **Historical Learning**: Chronicler Rahewin described the Kaiserpfalz as having a strong surrounding wall, a large fishpond, and an adjacent deer park. Players learn why medieval emperors travelled continuously across itinerant palaces (Reisekönigtum).
   - **Cooperative Challenge**: Collect and position 3 historical markers (Fortified Wall, Fishpond, Wildlife Park), use the rope anchor to assist teammates, and operate dual levers simultaneously to open the portcullis gate.

2. **Section 2: The Domus (Der Große Domus)**
   - **Environment**: The imperial residential palace hall, foundation blocks, wooden lifting mechanisms, and elevated sandstone ledges.
   - **Historical Learning**: The Domus was the principal imperial residential building with documented dimensions of approximately **25m length × 19m width × 19m height**.
   - **Cooperative Challenge**: Carry foundation stone blocks, coordinate rope climbing, and operate shared winch levers to breach the upper floor.

3. **Section 3: The Later Palace Chapel (Die Spätere Doppelkapelle)**
   - **Environment**: Romanesque/Gothic red sandstone arches, masonry fragments, and the double chapel layout.
   - **Historical Learning**: The double chapel was constructed in a later building phase under Emperor Frederick II (Barbarossa's grandson) in the 13th century, distinct from Barbarossa's original 12th-century structure.
   - **Cooperative Challenge**: Distinguish building phase fragments, position them in the altar socket, and ring the dual altar bells to reach the summit.

4. **Imperial Summit & Reconstruction Screen**:
   - Visual architectural reconstruction of the entire Kaiserpfalz complex.
   - Educational recap of all completed challenges and historical facts.

---

## 🤝 Cooperative Gameplay Mechanics

- **Rope Assistance**: Players at designated anchors can lower ropes (`[Interact]`). Teammates below can grab the rope and climb upward (`[Up] / [Down]`).
- **Shared Mechanisms (Dual Levers)**: Certain portcullises and gates require two players standing on Switch A and Switch B, holding the interaction button simultaneously for 1.2 seconds.
- **Cooperative Carrying**: Heavy stone blocks and historical markers can be carried together across platforms to their target sockets.
- **AI Helper Bots (Disconnect Protection)**: If a smartphone disconnects, a 5-second reconnection grace period begins. If connection is lost, an intelligent bot automatically takes over to follow waypoints, activate partner switches, and carry items so the team is never blocked. Control is immediately restored upon reconnection.

---

## 📱 Smartphone Controller Connection

The game connects to the university input relay (`scilab-0098.informatik.uni-kl.de`) using the credentials from `TOKEN.md`:

```powershell
$env:INPUT_MUX_URL = "https://scilab-0098.informatik.uni-kl.de"
$env:INPUT_MUX_TOKEN = "imx_..."
```

1. Launch the game:
   ```sh
   cargo run
   ```
2. The lobby displays **QR codes and URLs** for Player 1, Player 2, Player 3, and Player 4.
3. Players scan their respective QR code with their mobile phone camera (Safari/Chrome).
4. The smartphone displays a responsive touch gamepad:
   - **D-Pad (Left / Right / Up / Down)**: Move horizontally and climb ropes.
   - **[ A ]**: Jump.
   - **[ B ] / [ X ]**: Interact (lower rope / hold lever / carry object).
   - **[ Menu ]**: Start game from lobby / return.

---

## ⌨️ Local Keyboard Controls (PC Fallback)

The game is also fully playable without smartphones using local keyboard input:

| Action | Player 1 (Keyboard) | Player 2 (Keyboard) |
| :--- | :--- | :--- |
| **Move Left / Right** | `A` / `D` or `Left` / `Right` Arrows | `J` / `L` |
| **Jump** | `Space` / `W` / `Up Arrow` | `I` |
| **Interact / Rope / Lever** | `E` (Tap or Hold) | `K` (Tap or Hold) |
| **Quiz Answer (Checkpoint)** | `1`, `2`, `3`, `4` or `A`, `B`, `X`, `Y` | Phone buttons or PC keys |
| **Start / Confirm** | `Enter` or `Space` | `Menu` on controller |

---

## 🧪 Testing

Run the automated test suite verifying physics, collisions, shared mechanism synchronization, rope assistance, and bot takeover:

```sh
cargo test
```
