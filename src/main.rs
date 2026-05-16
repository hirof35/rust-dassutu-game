use eframe::egui;

// ==========================================
// データモデルの定義
// ==========================================
#[derive(Clone, PartialEq)]
enum GameScreen {
    Title,
    Game,
    Ending,
}

#[derive(Clone, PartialEq)]
enum GimmickState {
    Exploring,
    Gimmick,
}

#[derive(Clone, PartialEq)]
struct Item {
    name: String,
}

struct SceneObject {
    name: String,
    bounds: [f32; 4],
    item: Item,
    is_picked_up: bool,
}

struct Exit {
    bounds: [f32; 4],
    next_scene: String,
    required_item: Option<String>,
    is_password_gate: bool,
}

struct Scene {
    name: String,
    color: egui::Color32,
    exits: Vec<Exit>,
    objects: Vec<SceneObject>,
}

struct EscapeGameApp {
    screen: GameScreen,
    gimmick_state: GimmickState,
    current_scene_id: String,
    scenes: std::collections::HashMap<String, Scene>,
    inventory: Vec<Item>,
    selected_item_index: Option<usize>,
    message: String,
    digits: [u32; 4],
    answer: [u32; 4],
}

impl Default for EscapeGameApp {
    fn default() -> Self {
        let mut scenes = std::collections::HashMap::new();

        // 部屋A
        scenes.insert(
            "roomA".to_string(),
            Scene {
                name: "部屋A".to_string(),
                color: egui::Color32::from_rgb(30, 50, 100),
                exits: vec![
                    Exit { bounds: [700.0, 200.0, 100.0, 200.0], next_scene: "roomB".to_string(), required_item: None, is_password_gate: false },
                    Exit { bounds: [350.0, 50.0, 100.0, 100.0], next_scene: "roomC".to_string(), required_item: None, is_password_gate: false },
                    Exit { bounds: [50.0, 200.0, 120.0, 250.0], next_scene: "ENDING".to_string(), required_item: Some("電子カードキー".to_string()), is_password_gate: true },
                ],
                objects: vec![],
            },
        );

        // 部屋B
        scenes.insert(
            "roomB".to_string(),
            Scene {
                name: "部屋B".to_string(),
                color: egui::Color32::from_rgb(100, 30, 80),
                exits: vec![
                    Exit { bounds: [0.0, 200.0, 100.0, 200.0], next_scene: "roomA".to_string(), required_item: None, is_password_gate: false },
                ],
                objects: vec![
                    SceneObject {
                        name: "クローゼット".to_string(),
                        bounds: [400.0, 200.0, 150.0, 250.0],
                        item: Item { name: "電子カードキー".to_string() },
                        is_picked_up: false,
                    },
                ],
            },
        );

        // 部屋C
        scenes.insert(
            "roomC".to_string(),
            Scene {
                name: "部屋C".to_string(),
                color: egui::Color32::from_rgb(120, 80, 20),
                exits: vec![
                    Exit { bounds: [350.0, 450.0, 100.0, 50.0], next_scene: "roomA".to_string(), required_item: None, is_password_gate: false },
                ],
                objects: vec![
                    SceneObject {
                        name: "机の引き出し".to_string(),
                        bounds: [250.0, 300.0, 150.0, 100.0],
                        item: Item { name: "小さな鍵".to_string() },
                        is_picked_up: false,
                    },
                ],
            },
        );

        Self {
            screen: GameScreen::Title,
            gimmick_state: GimmickState::Exploring,
            current_scene_id: "roomA".to_string(),
            scenes,
            inventory: vec![],
            selected_item_index: None,
            message: "部屋A: ここから脱出しよう。奥と右に部屋があるようだ。".to_string(),
            digits: [0, 0, 0, 0],
            answer: [5, 9, 6, 3],
        }
    }
}

// ==========================================
// フォント設定関数（追加）
// ==========================================
fn setup_japanese_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // プロジェクトルートに配置した日本語フォントファイルをバイナリとして読み込む
    // ※お手持ちのフォントファイル名に書き換えてください（例: NotoSansJP-Regular.ttf）
    fonts.font_data.insert(
        "japanese_font".to_owned(),
        egui::FontData::from_owned(include_bytes!("../NotoSansJP-Regular.otf").to_vec()),
    );

    // プロポーショナル（比例間隔）とモノスペース（等幅）の両方に日本語フォントを最優先指定
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "japanese_font".to_owned());
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
        .insert(0, "japanese_font".to_owned());

    ctx.set_fonts(fonts);
}

// ==========================================
// GUI描画
// ==========================================
impl eframe::App for EscapeGameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            GameScreen::Title => self.draw_title(ctx),
            GameScreen::Game => self.draw_game(ctx),
            GameScreen::Ending => self.draw_ending(ctx),
        }
    }
}

impl EscapeGameApp {
    fn draw_title(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(150.0);
                ui.heading(egui::RichText::new("3 ROOMS ESCAPE (Rust日本語版)").font(egui::FontId::proportional(40.0)));
                ui.add_space(100.0);
                if ui.add(egui::Button::new("ゲームを開始する").min_size(egui::vec2(200.0, 50.0))).clicked() {
                    self.screen = GameScreen::Game;
                }
            });
        });
    }

    fn draw_game(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let current_scene = self.scenes.get_mut(&self.current_scene_id).unwrap();

            painter.rect_filled(
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 450.0)),
                0.0,
                current_scene.color,
            );

            painter.text(
                egui::pos2(20.0, 40.0),
                egui::Align2::LEFT_TOP,
                format!("【{}】", current_scene.name),
                egui::FontId::proportional(20.0),
                egui::Color32::WHITE,
            );

            if current_scene.name == "部屋C" {
                painter.text(
                    egui::pos2(450.0, 100.0),
                    egui::Align2::LEFT_TOP,
                    "壁の落書き: 「ごくろう（5963）さん」",
                    egui::FontId::proportional(16.0),
                    egui::Color32::YELLOW,
                );
            }

            let click_pos = ui.input(|i| {
                if i.pointer.any_pressed() { i.pointer.interact_pos() } else { None }
            });

            if self.gimmick_state == GimmickState::Exploring {
                for obj in &mut current_scene.objects {
                    if !obj.is_picked_up {
                        let rect = egui::Rect::from_min_size(egui::pos2(obj.bounds[0], obj.bounds[1]), egui::vec2(obj.bounds[2], obj.bounds[3]));
                        painter.rect_filled(rect, 0.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 150));
                        painter.text(rect.center(), egui::Align2::CENTER_CENTER, &obj.name, egui::FontId::proportional(14.0), egui::Color32::BLACK);

                        if let Some(pos) = click_pos {
                            if rect.contains(pos) {
                                if obj.name == "クローゼット" {
                                    if let Some(idx) = self.selected_item_index {
                                        if self.inventory[idx].name == "小さな鍵" {
                                            self.inventory.push(obj.item.clone());
                                            self.inventory.remove(idx);
                                            self.selected_item_index = None;
                                            obj.is_picked_up = true;
                                            self.message = format!("小さな鍵でクローゼットを開けた！「{}」を手に入れた！", obj.item.name);
                                        } else {
                                            self.message = "クローゼットには鍵がかかっている。".to_string();
                                        }
                                    } else {
                                        self.message = "クローゼットには鍵がかかっている。".to_string();
                                    }
                                } else if obj.name == "机の引き出し" {
                                    self.inventory.push(obj.item.clone());
                                    obj.is_picked_up = true;
                                    self.message = format!("引き出しから「{}」を手に入れた！", obj.item.name);
                                }
                            }
                        }
                    }
                }

                let mut next_scene_target = None;
                for exit in &current_scene.exits {
                    let rect = egui::Rect::from_min_size(egui::pos2(exit.bounds[0], exit.bounds[1]), egui::vec2(exit.bounds[2], exit.bounds[3]));
                    let color = if exit.required_item.is_some() {
                        egui::Color32::from_rgba_unmultiplied(255, 0, 0, 100)
                    } else {
                        egui::Color32::from_rgba_unmultiplied(0, 255, 0, 100)
                    };
                    painter.rect_filled(rect, 0.0, color);
                    let label = if exit.required_item.is_some() { "脱出扉" } else { "矢印" };
                    painter.text(rect.center(), egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(14.0), egui::Color32::WHITE);

                    if let Some(pos) = click_pos {
                        if rect.contains(pos) {
                            if let Some(ref req_item) = exit.required_item {
                                if let Some(idx) = self.selected_item_index {
                                    if self.inventory[idx].name == *req_item {
                                        if exit.is_password_gate {
                                            self.gimmick_state = GimmickState::Gimmick;
                                            self.message = "暗証番号を入力してください。".to_string();
                                        } else {
                                            next_scene_target = Some(exit.next_scene.clone());
                                        }
                                    } else {
                                        self.message = format!("扉のリーダーが赤く光っている。「{}」が必要だ。", req_item);
                                    }
                                } else {
                                    self.message = format!("扉のリーダーが赤く光っている。「{}」が必要だ。", req_item);
                                }
                            } else {
                                next_scene_target = Some(exit.next_scene.clone());
                            }
                        }
                    }
                }

                if let Some(next) = next_scene_target {
                    if next == "ENDING" {
                        self.screen = GameScreen::Ending;
                    } else {
                        self.current_scene_id = next;
                        let name = self.scenes.get(&self.current_scene_id).unwrap().name.clone();
                        self.message = format!("{}に移動しました。", name);
                    }
                }

            } else {
                // パスワード画面
                painter.rect_filled(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 450.0)), 0.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 230));
                painter.text(egui::pos2(400.0, 150.0), egui::Align2::CENTER_CENTER, "電子ロック: パスワードを入力", egui::FontId::proportional(24.0), egui::Color32::WHITE);

                let mut outside_click = click_pos.is_some();
                for i in 0..4 {
                    let x = 200.0 + (i as f32 * 110.0);
                    let draw_rect = egui::Rect::from_min_size(egui::pos2(x, 220.0), egui::vec2(80.0, 80.0));
                    painter.rect_filled(draw_rect, 0.0, egui::Color32::WHITE);
                    painter.text(draw_rect.center(), egui::Align2::CENTER_CENTER, self.digits[i].to_string(), egui::FontId::proportional(48.0), egui::Color32::BLACK);

                    if let Some(pos) = click_pos {
                        if draw_rect.contains(pos) {
                            self.digits[i] = (self.digits[i] + 1) % 10;
                            outside_click = false;

                            if self.digits == self.answer {
                                self.gimmick_state = GimmickState::Exploring;
                                if let Some(idx) = self.selected_item_index { self.inventory.remove(idx); }
                                self.selected_item_index = None;
                                self.screen = GameScreen::Ending;
                            }
                        }
                    }
                }

                if outside_click {
                    if let Some(pos) = click_pos {
                        if pos.y < 150.0 || pos.y > 350.0 {
                            self.gimmick_state = GimmickState::Exploring;
                            self.message = "入力を中断した。".to_string();
                        }
                    }
                }
            }
        });

        egui::TopBottomPanel::bottom("bottom_panel").resizable(false).default_height(150.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label(egui::RichText::new(&self.message).color(egui::Color32::LIGHT_BLUE).font(egui::FontId::proportional(16.0)));
            });

            ui.separator();
            ui.heading("【所持アイテム一覧】");
            ui.horizontal(|ui| {
                let mut next_selected = self.selected_item_index;
                for (i, item) in self.inventory.iter().enumerate() {
                    let is_selected = self.selected_item_index == Some(i);
                    let btn_text = if is_selected { format!("[ {} ]", item.name) } else { item.name.clone() };

                    let btn = egui::Button::new(btn_text)
                        .min_size(egui::vec2(120.0, 40.0))
                        .fill(if is_selected { egui::Color32::YELLOW } else { egui::Color32::GRAY });

                    if ui.add(btn).clicked() {
                        if is_selected {
                            next_selected = None;
                            self.message = "選択解除".to_string();
                        } else {
                            next_selected = Some(i);
                            self.message = format!("{} を選択中", item.name);
                        }
                    }
                }
                self.selected_item_index = next_selected;
            });
        });
    }

    fn draw_ending(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(150.0);
                ui.heading(egui::RichText::new("脱出成功！").color(egui::Color32::YELLOW).font(egui::FontId::proportional(50.0)));
                ui.add_space(100.0);
                if ui.add(egui::Button::new("タイトルへ戻る").min_size(egui::vec2(200.0, 50.0))).clicked() {
                    *self = EscapeGameApp::default();
                    self.screen = GameScreen::Title;
                }
            });
        });
    }
}

// ==========================================
// メイン関数
// ==========================================
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_resizable(false),
        ..Default::default()
    };
    
    eframe::run_native(
        "本格脱出ゲームエンジン - Rust日本語対応版",
        options,
        Box::new(|cc| {
            // アプリ起動時に日本語フォントを読み込む
            setup_japanese_fonts(&cc.egui_ctx);
            Box::new(EscapeGameApp::default())
        }),
    )
}