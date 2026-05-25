use sf_api::command::Command;
use sf_api::session::SimpleSession;
use std::sync::Mutex;
use tauri::{Emitter, Manager, State};

// ── Pet-Mapping ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
enum Klasse { Krieger, Magier, Kundschafter }

impl Klasse {
    fn konter(&self) -> Klasse {
        match self {
            Klasse::Krieger      => Klasse::Kundschafter,
            Klasse::Magier       => Klasse::Krieger,
            Klasse::Kundschafter => Klasse::Magier,
        }
    }
    fn als_text(&self) -> &str {
        match self {
            Klasse::Krieger      => "Krieger",
            Klasse::Magier       => "Magier",
            Klasse::Kundschafter => "Kundschafter",
        }
    }
}

fn pet_info(id: u32) -> Option<(&'static str, &'static str, &'static str, Klasse)> {
    match id {
        1  => Some(("Schlorps",    "Schatten", "Shadow", Klasse::Kundschafter)),
        2  => Some(("Buddemull",   "Schatten", "Shadow", Klasse::Krieger)),
        3  => Some(("Zahnaug",     "Schatten", "Shadow", Klasse::Krieger)),
        4  => Some(("Okulontakel", "Schatten", "Shadow", Klasse::Magier)),
        5  => Some(("Spynn",       "Schatten", "Shadow", Klasse::Magier)),
        6  => Some(("Jackobu",     "Schatten", "Shadow", Klasse::Magier)),
        7  => Some(("Shrimpfly",   "Schatten", "Shadow", Klasse::Kundschafter)),
        8  => Some(("Schnigrim",   "Schatten", "Shadow", Klasse::Kundschafter)),
        9  => Some(("Rubdikaz",    "Schatten", "Shadow", Klasse::Kundschafter)),
        10 => Some(("Mykon",       "Schatten", "Shadow", Klasse::Krieger)),
        11 => Some(("Anglion",     "Schatten", "Shadow", Klasse::Magier)),
        12 => Some(("Kucklitz",    "Schatten", "Shadow", Klasse::Magier)),
        13 => Some(("Flederadatz", "Schatten", "Shadow", Klasse::Kundschafter)),
        14 => Some(("Rosalynx",    "Schatten", "Shadow", Klasse::Kundschafter)),
        15 => Some(("Pharamum",    "Schatten", "Shadow", Klasse::Krieger)),
        16 => Some(("Nynnja",      "Schatten", "Shadow", Klasse::Krieger)),
        17 => Some(("Luchtablong", "Schatten", "Shadow", Klasse::Magier)),
        18 => Some(("Zornox",      "Schatten", "Shadow", Klasse::Krieger)),
        19 => Some(("Deiblatz",    "Schatten", "Shadow", Klasse::Krieger)),
        20 => Some(("Phoison",     "Schatten", "Shadow", Klasse::Kundschafter)),
        21 => Some(("Zossl",        "Licht", "Light", Klasse::Krieger)),
        22 => Some(("Quaklops",     "Licht", "Light", Klasse::Krieger)),
        23 => Some(("Tinck",        "Licht", "Light", Klasse::Magier)),
        24 => Some(("Witteblitz",   "Licht", "Light", Klasse::Magier)),
        25 => Some(("Niesat",       "Licht", "Light", Klasse::Kundschafter)),
        26 => Some(("Plutoid",      "Licht", "Light", Klasse::Kundschafter)),
        27 => Some(("Dschinntonik", "Licht", "Light", Klasse::Magier)),
        28 => Some(("Blaxta",       "Licht", "Light", Klasse::Krieger)),
        29 => Some(("Plinze",       "Licht", "Light", Klasse::Krieger)),
        30 => Some(("Tezzler",      "Licht", "Light", Klasse::Magier)),
        31 => Some(("Fusonn",       "Licht", "Light", Klasse::Magier)),
        32 => Some(("Bofux",        "Licht", "Light", Klasse::Kundschafter)),
        33 => Some(("Tiwitsch",     "Licht", "Light", Klasse::Kundschafter)),
        34 => Some(("Dafuk",        "Licht", "Light", Klasse::Magier)),
        35 => Some(("Mesmerit",     "Licht", "Light", Klasse::Magier)),
        36 => Some(("Antlar",       "Licht", "Light", Klasse::Krieger)),
        37 => Some(("Uuf",          "Licht", "Light", Klasse::Krieger)),
        38 => Some(("Kylrich",      "Licht", "Light", Klasse::Krieger)),
        39 => Some(("Heraldon",     "Licht", "Light", Klasse::Magier)),
        40 => Some(("Unikor",       "Licht", "Light", Klasse::Kundschafter)),
        41 => Some(("Mamoton",    "Erde", "Earth", Klasse::Krieger)),
        42 => Some(("Rangatang",  "Erde", "Earth", Klasse::Krieger)),
        43 => Some(("Lilipon",    "Erde", "Earth", Klasse::Kundschafter)),
        44 => Some(("Naghezan",   "Erde", "Earth", Klasse::Kundschafter)),
        45 => Some(("Groar",      "Erde", "Earth", Klasse::Krieger)),
        46 => Some(("Muscudon",   "Erde", "Earth", Klasse::Kundschafter)),
        47 => Some(("Apstok",     "Erde", "Earth", Klasse::Magier)),
        48 => Some(("Rambok",     "Erde", "Earth", Klasse::Magier)),
        49 => Some(("Urrf",       "Erde", "Earth", Klasse::Krieger)),
        50 => Some(("Mameloth",   "Erde", "Earth", Klasse::Krieger)),
        51 => Some(("Nahon",      "Erde", "Earth", Klasse::Kundschafter)),
        52 => Some(("Stonor",     "Erde", "Earth", Klasse::Krieger)),
        53 => Some(("Rofus",      "Erde", "Earth", Klasse::Kundschafter)),
        54 => Some(("Kloppdiwop", "Erde", "Earth", Klasse::Kundschafter)),
        55 => Some(("Waltschrek", "Erde", "Earth", Klasse::Magier)),
        56 => Some(("Scheer",     "Erde", "Earth", Klasse::Magier)),
        57 => Some(("Tauerbok",   "Erde", "Earth", Klasse::Magier)),
        58 => Some(("Kanockle",   "Erde", "Earth", Klasse::Krieger)),
        59 => Some(("Tricerabor", "Erde", "Earth", Klasse::Krieger)),
        60 => Some(("Mauhel",     "Erde", "Earth", Klasse::Krieger)),
        61 => Some(("Ratzfratz",   "Feuer", "Fire", Klasse::Kundschafter)),
        62 => Some(("Gullps",      "Feuer", "Fire", Klasse::Kundschafter)),
        63 => Some(("Pyrophibus",  "Feuer", "Fire", Klasse::Krieger)),
        64 => Some(("Loderfleuch", "Feuer", "Fire", Klasse::Magier)),
        65 => Some(("Tektospei",   "Feuer", "Fire", Klasse::Magier)),
        66 => Some(("Pyroplant",   "Feuer", "Fire", Klasse::Kundschafter)),
        67 => Some(("Kokofo",      "Feuer", "Fire", Klasse::Kundschafter)),
        68 => Some(("Jappaelo",    "Feuer", "Fire", Klasse::Magier)),
        69 => Some(("Zischbum",    "Feuer", "Fire", Klasse::Krieger)),
        70 => Some(("Tikkon",      "Feuer", "Fire", Klasse::Magier)),
        71 => Some(("Fachan",      "Feuer", "Fire", Klasse::Magier)),
        72 => Some(("Feugel",      "Feuer", "Fire", Klasse::Kundschafter)),
        73 => Some(("Infernox",    "Feuer", "Fire", Klasse::Kundschafter)),
        74 => Some(("Sumsum",      "Feuer", "Fire", Klasse::Kundschafter)),
        75 => Some(("Dragopyr",    "Feuer", "Fire", Klasse::Kundschafter)),
        76 => Some(("Mantiflam",   "Feuer", "Fire", Klasse::Kundschafter)),
        77 => Some(("Flossnessel", "Feuer", "Fire", Klasse::Magier)),
        78 => Some(("Rocki",       "Feuer", "Fire", Klasse::Krieger)),
        79 => Some(("Zyngel",      "Feuer", "Fire", Klasse::Magier)),
        80 => Some(("Devastor",    "Feuer", "Fire", Klasse::Krieger)),
        81  => Some(("Goldi",      "Wasser", "Water", Klasse::Magier)),
        82  => Some(("Orkal",      "Wasser", "Water", Klasse::Krieger)),
        83  => Some(("Okodil",     "Wasser", "Water", Klasse::Krieger)),
        84  => Some(("Pung",       "Wasser", "Water", Klasse::Krieger)),
        85  => Some(("Walruphin",  "Wasser", "Water", Klasse::Krieger)),
        86  => Some(("Schnogg",    "Wasser", "Water", Klasse::Kundschafter)),
        87  => Some(("Aguaphant",  "Wasser", "Water", Klasse::Krieger)),
        88  => Some(("Naar",       "Wasser", "Water", Klasse::Kundschafter)),
        89  => Some(("Tinntak",    "Wasser", "Water", Klasse::Kundschafter)),
        90  => Some(("Ewilgryn",   "Wasser", "Water", Klasse::Krieger)),
        91  => Some(("Separd",     "Wasser", "Water", Klasse::Magier)),
        92  => Some(("Mussli",     "Wasser", "Water", Klasse::Magier)),
        93  => Some(("Mingho",     "Wasser", "Water", Klasse::Magier)),
        94  => Some(("Untabis",    "Wasser", "Water", Klasse::Krieger)),
        95  => Some(("Nixoid",     "Wasser", "Water", Klasse::Magier)),
        96  => Some(("Fluoch",     "Wasser", "Water", Klasse::Magier)),
        97  => Some(("Patortel",   "Wasser", "Water", Klasse::Krieger)),
        98  => Some(("Unnda",      "Wasser", "Water", Klasse::Magier)),
        99  => Some(("Tritostach", "Wasser", "Water", Klasse::Krieger)),
        100 => Some(("Hydrospir",  "Wasser", "Water", Klasse::Kundschafter)),
        _   => None,
    }
}

// ── App State ─────────────────────────────────────────────────────────────────

struct AppState {
    sessions:    Mutex<Vec<std::sync::Arc<tokio::sync::Mutex<SimpleSession>>>>,
    gs_json:     Mutex<Option<serde_json::Value>>,
    member_data: Mutex<Vec<MemberData>>,
    char_index:  Mutex<usize>,
    own_name:    Mutex<String>,
    pending_update: Mutex<Option<tauri_plugin_updater::Update>>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct MemberData {
    #[allow(dead_code)]
    name:         String,
    level:        u32,
    strength:     u32,
    dexterity:    u32,
    intelligence: u32,
}

// ── Response-Typen ────────────────────────────────────────────────────────────

#[derive(serde::Serialize)]
struct LoginResponse {
    characters: Vec<String>,
}

#[derive(serde::Serialize)]
struct GuildInfo {
    active_pet_name:    String,
    active_pet_element: String,
    active_pet_class:   String,
    active_pet_id:      u32,
    hydra_life_pct:     u64,
    hydra_life_cur:     u64,
    hydra_life_max:     u64,
}

#[derive(serde::Serialize, Clone)]
struct MemberProgress {
    current: usize,
    total:   usize,
    name:    String,
}

#[derive(serde::Serialize)]
struct ElementStats {
    element:   String,
    avg_level: f64,
    sum_attr:  u32,
}

#[derive(serde::Serialize)]
struct Recommendation {
    pet_name:        String,
    pet_element:     String,
    pet_id:          u32,
    pet_level:       u32,
    avg_guild_level: f64,
    sum_attr:        u32,
    no_change:       bool,
    hydra_class:     String,
    counter_class:   String,
    element_stats:   Vec<ElementStats>,
}

// ── Pet-Extraktion ────────────────────────────────────────────────────────────

const BUNDLE_URL: &str = "https://cdn.sfgame.net/res/sfgame3/assets/sfprod30.500.465/WebGL/petmediumsprites?h=d71349b781f26897643698010f492cd1";

fn pets_dir() -> Result<std::path::PathBuf, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    Ok(exe.parent().unwrap().join("img").join("pets"))
}

#[tauri::command]
fn get_pets_base_path() -> String {
    pets_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}

#[tauri::command]
fn check_pets_exist() -> bool {
    if let Ok(dir) = pets_dir() {
        (1..=100u32).all(|i| dir.join(format!("pet_{:03}.png", i)).exists())
    } else {
        false
    }
}

#[tauri::command]
async fn extract_pets(window: tauri::Window) -> Result<String, String> {
    use unity_asset_decode::file::{load_unity_file_from_memory, UnityFile};
    use unity_asset_decode::object::UnityObject;
    use unity_asset_decode::texture::Texture2DConverter;
    use unity_asset_decode::unity_version::UnityVersion;

    let _ = window.emit("pet_progress", "Lade Bundle...").ok();

    // Bundle herunterladen
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build().map_err(|e| format!("HTTP-Client Fehler: {}", e))?;
    let bytes = client.get(BUNDLE_URL)
        .send().await.map_err(|e| format!("Download fehlgeschlagen: {}", e))?
        .bytes().await.map_err(|e| format!("Bytes lesen fehlgeschlagen: {}", e))?
        .to_vec();

    let _ = window.emit("pet_progress",
        format!("Bundle geladen ({} KB) – Parse...", bytes.len() / 1024)).ok();

    // Bundle parsen
    let unity_file = load_unity_file_from_memory(bytes)
        .map_err(|e| format!("Bundle-Parse fehlgeschlagen: {:?}", e))?;

    let version = UnityVersion::parse_version("6000.3.11f1")
        .map_err(|e| format!("UnityVersion Parse: {:?}", e))?;

    let bundle = match unity_file {
        UnityFile::AssetBundle(b) => b,
        UnityFile::SerializedFile(_) => return Err("Unerwartetes SerializedFile-Format".into()),
        UnityFile::WebFile(_)        => return Err("Unerwartetes WebFile-Format".into()),
    };

    let tex_conv    = Texture2DConverter::new(version);

    // Pass 1: Alle Objekte sammeln
    // - Texture2D für den Atlas
    // - ALLE Sprites nach PathID (werden vom MonoBehaviour referenziert)
    // - MonoBehaviour "PetMediumSprites" (gibt die korrekte pet_id-Reihenfolge)
    // - SpriteAtlas "PetsMediumAtlas" (enthält echte textureRect pro Sprite)
    let mut raw_texture: Option<unity_asset_decode::texture::Texture2D> = None;
    let mut sprites_by_id: std::collections::HashMap<i64, UnityObject> = std::collections::HashMap::new();
    let mut mono_obj:  Option<UnityObject> = None;
    let mut atlas_obj: Option<UnityObject> = None;

    for sf in &bundle.assets {
        for info in &sf.objects {
            let obj = match UnityObject::from_serialized_file(sf, info) {
                Ok(o)  => o,
                Err(_) => continue,
            };
            match obj.class_name() {
                "Texture2D" => {
                    if raw_texture.is_none() {
                        if let Ok(tex) = tex_conv.from_unity_object(&obj) {
                            raw_texture = Some(tex);
                        }
                    }
                }
                "Sprite" => {
                    sprites_by_id.insert(info.path_id, obj);
                }
                "MonoBehaviour" => {
                    if obj.name() == Some("PetMediumSprites".to_string()) {
                        mono_obj = Some(obj);
                    }
                }
                "SpriteAtlas" => {
                    atlas_obj = Some(obj);
                }
                _ => {}
            }
        }
    }

    let texture = raw_texture
        .ok_or("Keine Texture2D im Bundle gefunden")?;

    // ── SpriteAtlas m_RenderDataMap aufbauen ─────────────────────────────────
    // Key: (guid_0, guid_1, guid_2, guid_3, local_id) → (abs_x, abs_y_unity, w, h)
    // guid = 4 × u32, local_id = i64 (aus m_RenderDataKey jedes Sprites)
    type RenderKey = (u32, u32, u32, u32, i64);
    let mut render_map: std::collections::HashMap<RenderKey, (f32, f32, f32, f32)> =
        std::collections::HashMap::new();

    if let Some(ref sa) = atlas_obj {
        use unity_asset_core::unity_value::UnityValue;

        fn get_f32(v: &UnityValue) -> f32 {
            match v {
                UnityValue::Float(f)   => *f as f32,
                UnityValue::Integer(i) => *i as f32,
                _ => 0.0,
            }
        }
        fn get_i64(v: &UnityValue) -> i64 {
            match v {
                UnityValue::Integer(i) => *i,
                UnityValue::Float(f)   => *f as i64,
                _ => 0,
            }
        }
        fn get_u32(v: &UnityValue) -> u32 {
            match v {
                UnityValue::Integer(i) => *i as u32,
                _ => 0,
            }
        }

        if let Some(UnityValue::Array(entries)) = sa.get("m_RenderDataMap") {
            for entry in entries {
                // Jeder Eintrag = [key_pair, value_object]
                let pair = match entry {
                    UnityValue::Array(p) => p,
                    _ => continue,
                };
                if pair.len() < 2 { continue; }

                // Key: pair[0] = [GUID-Object, localID]
                let (g0, g1, g2, g3, local_id) = match &pair[0] {
                    UnityValue::Array(kp) if kp.len() >= 2 => {
                        let guid = match &kp[0] {
                            UnityValue::Object(m) => {
                                let d0 = m.get("data[0]").map(get_u32).unwrap_or(0);
                                let d1 = m.get("data[1]").map(get_u32).unwrap_or(0);
                                let d2 = m.get("data[2]").map(get_u32).unwrap_or(0);
                                let d3 = m.get("data[3]").map(get_u32).unwrap_or(0);
                                (d0, d1, d2, d3)
                            }
                            _ => (0, 0, 0, 0),
                        };
                        let lid = get_i64(&kp[1]);
                        (guid.0, guid.1, guid.2, guid.3, lid)
                    }
                    _ => continue,
                };

                // Value: pair[1] = SpriteAtlasData Object
                let val = match &pair[1] {
                    UnityValue::Object(m) => m,
                    _ => continue,
                };
                let tr = match val.get("textureRect") {
                    Some(UnityValue::Object(r)) => {
                        let x = r.get("x").map(get_f32).unwrap_or(0.0);
                        let y = r.get("y").map(get_f32).unwrap_or(0.0);
                        let w = r.get("width").map(get_f32).unwrap_or(186.0);
                        let h = r.get("height").map(get_f32).unwrap_or(186.0);
                        (x, y, w, h)
                    }
                    _ => continue,
                };

                render_map.insert((g0, g1, g2, g3, local_id), tr);
            }
        }
    }

    log::info!("SpriteAtlas render_map: {} Einträge geladen", render_map.len());

    // Geordnete Sprite-PathIDs aus MonoBehaviour.petSmall lesen
    let ordered_ids: Vec<i64> = {
        use unity_asset_core::unity_value::UnityValue;
        let mb = mono_obj.ok_or(
            "MonoBehaviour 'PetMediumSprites' nicht gefunden! Bundle-URL könnte veraltet sein."
        )?;
        match mb.get("petSmall") {
            Some(UnityValue::Array(arr)) => {
                arr.iter().filter_map(|entry| {
                    if let UnityValue::Object(map) = entry {
                        if let Some(UnityValue::Integer(id)) = map.get("m_PathID") {
                            return Some(*id);
                        }
                    }
                    None
                }).collect()
            }
            _ => return Err("MonoBehaviour.petSmall nicht lesbar".into()),
        }
    };

    if ordered_ids.is_empty() {
        return Err("petSmall-Array ist leer".into());
    }

    let _ = window.emit("pet_progress",
        format!("{} Pet-Sprites gefunden – dekodiere Atlas (DXT5Crunched)...",
            ordered_ids.len())).ok();

    // Atlas-Textur mit texture2ddecoder dekodieren (DXT5Crunched = Unity Crunch + DXT5)
    let atlas_w = texture.width  as usize;
    let atlas_h = texture.height as usize;
    let mut atlas_pixels = vec![0u32; atlas_w * atlas_h];


    extern "C" {
        fn crunch_unpack(data: *const u8, size: u32,
                         out_data: *mut *mut u8, out_size: *mut u32) -> bool;
        fn crunch_free(ptr: *mut u8);
    }

    let _ = window.emit("pet_progress", "Starte Crunch-Dekomprimierung...").ok();

    // Schritt 1: Crunch → rohe DXT5-Bytes (via nativer C++-Bibliothek)
    // In spawn_blocking ausführen damit der async-Thread nicht blockiert wird
    let image_data = texture.image_data.clone();
    let crunch_result = tokio::time::timeout(
        std::time::Duration::from_secs(60),
        tokio::task::spawn_blocking(move || {
            let mut ptr: *mut u8 = std::ptr::null_mut();
            let mut sz: u32 = 0;
            let ok = unsafe {
                crunch_unpack(image_data.as_ptr(), image_data.len() as u32,
                              &mut ptr, &mut sz)
            };
            if ok && !ptr.is_null() {
                Ok((ptr as usize, sz as usize))
            } else {
                Err("Unity-Crunch-Dekomprimierung fehlgeschlagen".to_string())
            }
        })
    ).await
        .map_err(|_| "Crunch-Dekomprimierung Timeout nach 60s".to_string())?
        .map_err(|e| format!("spawn_blocking Fehler: {:?}", e))?
        .map_err(|e| e)?;

    let (dxt5_raw_ptr, dxt5_size) = crunch_result;
    let dxt5_ptr = dxt5_raw_ptr as *mut u8;
    let _ = window.emit("pet_progress", "Crunch fertig – dekodiere DXT5...").ok();

    // Schritt 2: DXT5/BC3 → BGRA
    {
        let dxt5 = unsafe { std::slice::from_raw_parts(dxt5_ptr, dxt5_size) };
        let bw = (atlas_w + 3) / 4;
        let bh = (atlas_h + 3) / 4;
        for by in 0..bh {
            for bx in 0..bw {
                let block = &dxt5[(by * bw + bx) * 16..];
                // Alpha-Block
                let a0 = block[0] as u32; let a1 = block[1] as u32;
                let mut abits: u64 = 0;
                for k in 0..6usize { abits |= (block[2+k] as u64) << (k*8); }
                let alphas: [u32;8] = if a0 > a1 {
                    [a0,a1,(6*a0+a1)/7,(5*a0+2*a1)/7,(4*a0+3*a1)/7,(3*a0+4*a1)/7,(2*a0+5*a1)/7,(a0+6*a1)/7]
                } else {
                    [a0,a1,(4*a0+a1)/5,(3*a0+2*a1)/5,(2*a0+3*a1)/5,(a0+4*a1)/5,0,255]
                };
                // Farb-Block
                let c0 = (block[8] as u32) | ((block[9] as u32)<<8);
                let c1 = (block[10] as u32) | ((block[11] as u32)<<8);
                let r0 = ((c0>>11)&31)*255/31; let g0 = ((c0>>5)&63)*255/63; let b0 = (c0&31)*255/31;
                let r1 = ((c1>>11)&31)*255/31; let g1 = ((c1>>5)&63)*255/63; let b1 = (c1&31)*255/31;
                let colors: [(u32,u32,u32);4] = if c0 > c1 {
                    [(r0,g0,b0),(r1,g1,b1),((2*r0+r1)/3,(2*g0+g1)/3,(2*b0+b1)/3),((r0+2*r1)/3,(g0+2*g1)/3,(b0+2*b1)/3)]
                } else {
                    [(r0,g0,b0),(r1,g1,b1),((r0+r1)/2,(g0+g1)/2,(b0+b1)/2),(0,0,0)]
                };
                let cbits = (block[12] as u32)|((block[13] as u32)<<8)|((block[14] as u32)<<16)|((block[15] as u32)<<24);
                for py in 0..4usize { for px in 0..4usize {
                    let (ix,iy) = (bx*4+px, by*4+py);
                    if ix>=atlas_w || iy>=atlas_h { continue; }
                    let pi = py*4+px;
                    let a = alphas[((abits>>(pi*3))&7) as usize];
                    let (r,g,b) = colors[((cbits>>(pi*2))&3) as usize];
                    atlas_pixels[iy*atlas_w+ix] = b|(g<<8)|(r<<16)|(a<<24);
                }}
            }
        }
    }
    unsafe { crunch_free(dxt5_ptr); }

    // BGRA32 → RGBA8 umrechnen (texture2ddecoder gibt BGRA aus)
    let atlas_rgba: Vec<u8> = atlas_pixels.iter()
        .flat_map(|&px| {
            let b = (px        & 0xFF) as u8;
            let g = ((px >> 8)  & 0xFF) as u8;
            let r = ((px >> 16) & 0xFF) as u8;
            let a = ((px >> 24) & 0xFF) as u8;
            [r, g, b, a]
        })
        .collect();

    // Atlas-Bild aufbauen (Unity: Y=0 unten → nach flip: Y=0 oben)
    let atlas_raw = image::RgbaImage::from_raw(atlas_w as u32, atlas_h as u32, atlas_rgba)
        .ok_or("Atlas-Bild konnte nicht erstellt werden")?;
    let _atlas = image::imageops::flip_vertical(&atlas_raw);

    let _ = window.emit("pet_progress", "Atlas dekodiert – schneide Sprites aus...").ok();

    let dir = pets_dir()?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let mut saved = 0u32;
    let total = ordered_ids.len();

    // Helper: Float-Wert aus UnityValue lesen (noch für render_map-Aufbau oben nötig)
    fn uv_float(val: &unity_asset_core::unity_value::UnityValue) -> f32 {
        use unity_asset_core::unity_value::UnityValue;
        match val {
            UnityValue::Float(f) => *f as f32,
            UnityValue::Integer(i) => *i as f32,
            _ => 0.0,
        }
    }
    let _ = uv_float; // suppress unused warning

    for (i, path_id) in ordered_ids.iter().enumerate() {
        let pet_id = (i + 1) as u32;

        let sprite_obj = match sprites_by_id.get(path_id) {
            Some(s) => s,
            None => {
                log::warn!("Pet {:03}: Sprite PathID={} nicht gefunden", pet_id, path_id);
                continue;
            }
        };
        let name = sprite_obj.name().unwrap_or_else(|| format!("path_{}", path_id));

        // m_RenderDataKey des Sprites lesen → Atlas-Koordinaten nachschlagen
        let atlas_rect: Option<(f32, f32, f32, f32)> = {
            use unity_asset_core::unity_value::UnityValue;
            fn get_u32(v: &UnityValue) -> u32 {
                match v { UnityValue::Integer(i) => *i as u32, _ => 0 }
            }
            fn get_i64(v: &UnityValue) -> i64 {
                match v { UnityValue::Integer(i) => *i, _ => 0 }
            }
            sprite_obj.get("m_RenderDataKey")
                .and_then(|rk| match rk {
                    UnityValue::Array(pair) if pair.len() >= 2 => {
                        let (g0, g1, g2, g3) = match &pair[0] {
                            UnityValue::Object(m) => (
                                m.get("data[0]").map(get_u32).unwrap_or(0),
                                m.get("data[1]").map(get_u32).unwrap_or(0),
                                m.get("data[2]").map(get_u32).unwrap_or(0),
                                m.get("data[3]").map(get_u32).unwrap_or(0),
                            ),
                            _ => (0, 0, 0, 0),
                        };
                        let lid = get_i64(&pair[1]);
                        render_map.get(&(g0, g1, g2, g3, lid)).copied()
                    }
                    _ => None,
                })
        };

        let (abs_x, abs_y_unity, sw, sh) = match atlas_rect {
            Some((x, y, w, h)) => (x as u32, y, w as u32, h as u32),
            None => {
                log::warn!("Pet {:03} ({}): kein SpriteAtlas-Eintrag, übersprungen", pet_id, name);
                continue;
            }
        };

        let crop_y = abs_y_unity as u32;

        let sprite_raw = image::imageops::crop_imm(&atlas_raw, abs_x, crop_y, sw, sh).to_image();
        let sprite_img = image::imageops::flip_vertical(&sprite_raw);

        let out_path = dir.join(format!("pet_{:03}.png", pet_id));
        if let Err(e) = sprite_img.save(&out_path) {
            log::warn!("Pet {:03}: Speichern fehlgeschlagen: {}", pet_id, e);
            continue;
        }

        saved += 1;
        let _ = window.emit("pet_progress",
            format!("{}/{} gespeichert", saved, total)).ok();
    }

    Ok(format!("{}/{} Pet-Bilder gespeichert nach:\n{}", saved, total, dir.display()))
}

// ── Tauri Commands ────────────────────────────────────────────────────────────

#[tauri::command]
async fn login(
    username: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<LoginResponse, String> {
    let sessions = SimpleSession::login_sf_account(&username, &password)
        .await
        .map_err(|e| format!("Login fehlgeschlagen: {:?}", e))?;

    let characters: Vec<String> = sessions
        .iter()
        .map(|s| s.username().to_string())
        .collect();

    *state.sessions.lock().unwrap() = sessions
        .into_iter()
        .map(|s| std::sync::Arc::new(tokio::sync::Mutex::new(s)))
        .collect();
    *state.gs_json.lock().unwrap()  = None;
    *state.member_data.lock().unwrap() = Vec::new();

    Ok(LoginResponse { characters })
}

#[tauri::command]
async fn load_guild(
    char_index: usize,
    state: State<'_, AppState>,
) -> Result<GuildInfo, String> {
    let session = state.sessions.lock().unwrap()
        .get(char_index)
        .ok_or("Ungültiger Charakter-Index")?
        .clone();
    let gs_json = {
        let mut guard = session.lock().await;
        let gs = guard.send_command(Command::Update)
            .await
            .map_err(|e| format!("Fehler beim Laden: {:?}", e))?;
        serde_json::to_value(&gs).unwrap()
    };

    {
        let basis     = &gs_json["character"]["attribute_basis"];
        let own_level = gs_json["character"]["level"].as_u64().unwrap_or(0) as u32;
        let own_name  = gs_json["character"]["name"].as_str().unwrap_or("").to_string();
        *state.own_name.lock().unwrap() = own_name.clone();
        *state.member_data.lock().unwrap() = vec![MemberData {
            name:         own_name,
            level:        own_level,
            strength:     basis["Strength"].as_u64().unwrap_or(0) as u32,
            dexterity:    basis["Dexterity"].as_u64().unwrap_or(0) as u32,
            intelligence: basis["Intelligence"].as_u64().unwrap_or(0) as u32,
        }];
    }

    *state.char_index.lock().unwrap() = char_index;

    let active_pet_id = gs_json["guild"]["pet_id"].as_u64().unwrap_or(0) as u32;
    let (pet_name, pet_elem, _, pet_class) = if active_pet_id == 0 {
        ("Kein Gildenpet", "—", "—", Klasse::Krieger)
    } else {
        pet_info(active_pet_id).unwrap_or(("Unbekannt", "?", "?", Klasse::Krieger))
    };

    let (hydra_pct, hydra_cur, hydra_max) =
        if let Some(hydra) = gs_json["guild"]["hydra"].as_object() {
            let cur = hydra["current_life"].as_u64().unwrap_or(0);
            let max = hydra["max_life"].as_u64().unwrap_or(0).max(1);
            (cur * 100 / max, cur, max)
        } else {
            (0, 0, 1)
        };

    *state.gs_json.lock().unwrap() = Some(gs_json);

    Ok(GuildInfo {
        active_pet_name:    pet_name.to_string(),
        active_pet_element: pet_elem.to_string(),
        active_pet_class:   pet_class.als_text().to_string(),
        active_pet_id,
        hydra_life_pct:     hydra_pct,
        hydra_life_cur:     hydra_cur,
        hydra_life_max:     hydra_max,
    })
}

#[tauri::command]
async fn load_members(
    state: State<'_, AppState>,
    window: tauri::Window,
) -> Result<usize, String> {
    let members_raw: Vec<(String, u32)> = {
        let gs_json = state.gs_json.lock().unwrap();
        let gs = gs_json.as_ref().ok_or("Kein GameState geladen")?;
        gs["guild"]["members"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|m| {
                let name  = m["name"].as_str()?.to_string();
                let level = m["level"].as_u64()? as u32;
                Some((name, level))
            })
            .collect()
    };

    let total      = members_raw.len();
    let own_name   = state.own_name.lock().unwrap().to_lowercase();
    let char_index = *state.char_index.lock().unwrap();

    for (i, (name, level)) in members_raw.iter().enumerate() {
        let _ = window.emit("member_progress", MemberProgress {
            current: i + 1,
            total,
            name: name.clone(),
        });

        if name.to_lowercase() == own_name {
            continue;
        }

        let session = state.sessions.lock().unwrap()
            .get(char_index)
            .map(|s| s.clone());
        let gs2_json_opt = if let Some(s) = session {
            let mut guard = s.lock().await;
            match guard.send_command(Command::ViewPlayer { ident: name.clone() }).await {
                Ok(gs2) => Some(serde_json::to_value(&gs2).unwrap()),
                Err(_)  => None,
            }
        } else { None };

        if let Some(gs2_json) = gs2_json_opt {
            let lookup = &gs2_json["lookup"]["players"];
            if let Some(obj) = lookup.as_object() {
                for (_, player) in obj {
                    let pname = player["name"].as_str().unwrap_or("");
                    if pname.to_lowercase() == name.to_lowercase() {
                        let basis = &player["attribute_basis"];
                        state.member_data.lock().unwrap().push(MemberData {
                            name:         name.clone(),
                            level:        *level,
                            strength:     basis["Strength"].as_u64().unwrap_or(0) as u32,
                            dexterity:    basis["Dexterity"].as_u64().unwrap_or(0) as u32,
                            intelligence: basis["Intelligence"].as_u64().unwrap_or(0) as u32,
                        });
                        break;
                    }
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    Ok(state.member_data.lock().unwrap().len())
}

#[tauri::command]
fn get_recommendation(
    hydra_class_num: u32,
    state: State<'_, AppState>,
) -> Result<Recommendation, String> {
    let (hydra_name, hydra_gruppe) = match hydra_class_num {
        1  => ("Krieger",       Klasse::Krieger),
        2  => ("Berserker",     Klasse::Krieger),
        3  => ("Kampfmagier",   Klasse::Krieger),
        4  => ("Paladin",       Klasse::Krieger),
        5  => ("Kundschafter",  Klasse::Kundschafter),
        6  => ("Assassine",     Klasse::Kundschafter),
        7  => ("Dämonenjäger",  Klasse::Kundschafter),
        8  => ("Magier",        Klasse::Magier),
        9  => ("Barde",         Klasse::Magier),
        10 => ("Nekromant",     Klasse::Magier),
        11 => ("Druide",        Klasse::Magier),
        _  => return Err("Ungültige Hydra-Klasse".to_string()),
    };

    let konter      = hydra_gruppe.konter();
    let member_data = state.member_data.lock().unwrap();
    let gs_json     = state.gs_json.lock().unwrap();
    let gs          = gs_json.as_ref().ok_or("Kein GameState geladen")?;

    let berechne_top25 = |attr_fn: &dyn Fn(&MemberData) -> u32| -> (f64, u32) {
        let mut sorted: Vec<&MemberData> = member_data.iter().collect();
        sorted.sort_by(|a, b| attr_fn(b).cmp(&attr_fn(a)));
        sorted.truncate(25);
        let divisor = if sorted.is_empty() { 1.0 } else { 25.0 };
        let avg_level = sorted.iter().map(|m| m.level as f64).sum::<f64>() / divisor;
        let sum_attr: u32 = sorted.iter().map(|m| attr_fn(m)).sum();
        (avg_level, sum_attr)
    };

    let (wasser_lvl, wasser_str) = berechne_top25(&|m: &MemberData| m.strength);
    let (licht_lvl,  licht_dex)  = berechne_top25(&|m: &MemberData| m.dexterity);
    let (erde_lvl,   erde_int)   = berechne_top25(&|m: &MemberData| m.intelligence);

    let element_stats = vec![
        ElementStats { element: "Krieger (Stärke)".to_string(),       avg_level: wasser_lvl, sum_attr: wasser_str },
        ElementStats { element: "Kundschafter (Geschick)".to_string(), avg_level: licht_lvl,  sum_attr: licht_dex  },
        ElementStats { element: "Magier (Intelligenz)".to_string(),   avg_level: erde_lvl,   sum_attr: erde_int   },
    ];

    let aktive_pet_id = gs["guild"]["pet_id"].as_u64().unwrap_or(0) as u32;

    // Alle 5 Habitate durchsuchen – Feuer und Schatten mit einbeziehen.
    // Attribut-Mapping nach Pet-KLASSE (laut Spielwerten bestätigt):
    // Kundschafter=Geschick(licht_dex), Krieger=Stärke(wasser_str), Magier=Intelligenz(erde_int)
    let attr_fuer_klasse = |klasse: &Klasse| -> (f64, u32) {
        match klasse {
            Klasse::Kundschafter => (licht_lvl,  licht_dex),
            Klasse::Krieger      => (wasser_lvl, wasser_str),
            Klasse::Magier       => (erde_lvl,   erde_int),
        }
    };

    let elemente = [
        ("Light",  "Licht"),
        ("Earth",  "Erde"),
        ("Water",  "Wasser"),
        ("Fire",   "Feuer"),
        ("Shadow", "Schatten"),
    ];

    let mut beste_sum   = 0u32;
    let mut beste_avg   = -1.0_f64;
    let mut beste_id    = 0u32;
    let mut bestes_lvl  = 0u32;
    let mut bestes_elem = "";
    let mut bestes_name = "";
    let mut bester_sum  = 0u32;

    for (elem_en, elem_de) in &elemente {
        if let Some(pets) = gs["pets"]["habitats"][elem_en]["pets"].as_array() {
            for pet in pets {
                let id  = pet["id"].as_u64().unwrap_or(0) as u32;
                let lvl = pet["level"].as_u64().unwrap_or(0) as u32;
                if lvl < 100 { continue; }
                if let Some((name, _, _, klasse)) = pet_info(id) {
                    if klasse != konter { continue; }
                    let (avg_lvl, sum_a) = attr_fuer_klasse(&klasse);
                    let besser = sum_a > beste_sum
                        || (sum_a == beste_sum && avg_lvl > beste_avg);
                    if besser {
                        beste_sum   = sum_a;
                        beste_avg   = avg_lvl;
                        beste_id    = id;
                        bestes_lvl  = lvl;
                        bestes_elem = elem_de;
                        bestes_name = name;
                        bester_sum  = sum_a;
                    }
                }
            }
        }
    }

    if beste_id == 0 {
        return Err(format!(
            "Kein passendes {}-Pet mit Level >= 100 gefunden.",
            konter.als_text()
        ));
    }

    // Kein Wechsel nötig wenn das aktive Pet bereits die richtige Konter-Klasse hat
    let akt_ist_konter = pet_info(aktive_pet_id)
        .map(|(_, _, _, k)| k == konter)
        .unwrap_or(false);
    let no_change = beste_id == aktive_pet_id || akt_ist_konter;

    // Bei "kein Wechsel": aktuelles Pet anzeigen, nicht das zufällig erste gefundene
    let (anzeige_id, anzeige_name, anzeige_elem, anzeige_lvl, anzeige_sum, anzeige_avg) =
        if no_change {
            if let Some((n, e, _, _)) = pet_info(aktive_pet_id) {
                let lvl = {
                    let all_elems = ["Light","Earth","Water","Fire","Shadow"];
                    let mut found_lvl = bestes_lvl;
                    'outer: for elem_en in &all_elems {
                        if let Some(pets) = gs["pets"]["habitats"][elem_en]["pets"].as_array() {
                            for pet in pets {
                                if pet["id"].as_u64().unwrap_or(0) as u32 == aktive_pet_id {
                                    found_lvl = pet["level"].as_u64().unwrap_or(0) as u32;
                                    break 'outer;
                                }
                            }
                        }
                    }
                    found_lvl
                };
                let (avg, sum) = attr_fuer_klasse(&konter);
                (aktive_pet_id, n, e, lvl, sum, avg)
            } else {
                (beste_id, bestes_name, bestes_elem, bestes_lvl, bester_sum, beste_avg)
            }
        } else {
            (beste_id, bestes_name, bestes_elem, bestes_lvl, bester_sum, beste_avg)
        };

    Ok(Recommendation {
        pet_name:        anzeige_name.to_string(),
        pet_element:     anzeige_elem.to_string(),
        pet_id:          anzeige_id,
        pet_level:       anzeige_lvl,
        avg_guild_level: anzeige_avg,
        sum_attr:        anzeige_sum,
        no_change,
        hydra_class:     hydra_name.to_string(),
        counter_class:   konter.als_text().to_string(),
        element_stats,
    })
}

#[tauri::command]
async fn logout(state: State<'_, AppState>) -> Result<(), String> {
    *state.sessions.lock().unwrap()    = Vec::new();
    *state.gs_json.lock().unwrap()     = None;
    *state.member_data.lock().unwrap() = Vec::new();
    *state.char_index.lock().unwrap()  = 0;
    *state.own_name.lock().unwrap()    = String::new();
    Ok(())
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn check_update_pending(state: State<'_, AppState>) -> Option<serde_json::Value> {
    let lock = state.pending_update.lock().unwrap();
    if let Some(ref update) = *lock {
        Some(serde_json::json!({
            "version": update.version,
            "notes": update.body.clone().unwrap_or_default(),
        }))
    } else {
        None
    }
}

#[tauri::command]
async fn install_update(state: State<'_, AppState>) -> Result<(), String> {
    let update = state.pending_update.lock().unwrap().take();
    if let Some(update) = update {
        update.download_and_install(|_, _| {}, || {})
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ── Einstiegspunkt ────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(AppState {
            sessions:       Mutex::new(Vec::new()),
            gs_json:        Mutex::new(None),
            member_data:    Mutex::new(Vec::new()),
            char_index:     Mutex::new(0),
            own_name:       Mutex::new(String::new()),
            pending_update: Mutex::new(None),
        })
        .setup(|app| {
            // Versionsnummer in Titelleiste setzen
            let version = env!("CARGO_PKG_VERSION");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_title(&format!("SF Hydra Advisor v{}", version));
            }
// Logging: Debug=Info, Release=Warn
            {
                let level = if cfg!(debug_assertions) {
                    log::LevelFilter::Info
                } else {
                    log::LevelFilter::Warn
                };
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(level)
                        .build(),
                )?;
            }
            // Updater: beim Start im Hintergrund auf neue Version prüfen
            #[cfg(not(debug_assertions))]
            {
                use tauri_plugin_updater::UpdaterExt;
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    // Warten bis Frontend bereit ist
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    if let Ok(updater) = handle.updater() {
                        if let Ok(Some(update)) = updater.check().await {
                            let version = update.version.clone();
                            let notes   = update.body.clone().unwrap_or_default();
                            *handle.state::<AppState>().pending_update.lock().unwrap()
                                = Some(update);
                            let _ = handle.emit("update-available", serde_json::json!({
                                "version": version,
                                "notes":   notes,
                            }));
                        }
                    }
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            login,
            load_guild,
            load_members,
            get_recommendation,
            logout,
            get_pets_base_path,
            check_pets_exist,
            extract_pets,
            install_update,
            check_update_pending,
            get_app_version,
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten der App");
}
