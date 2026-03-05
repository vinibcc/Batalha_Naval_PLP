use godot::prelude::*;
use godot::classes::{Node2D, INode2D, InputEvent, InputEventMouseButton, TileMapLayer};
use godot::global::MouseButton;

struct BatalhaNavalExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BatalhaNavalExtension {}

const BOARD_SIZE: usize = 10;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BattleController {
    player_board: [[u8; BOARD_SIZE]; BOARD_SIZE],
    enemy_board: [[u8; BOARD_SIZE]; BOARD_SIZE],
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for BattleController {
    fn init(base: Base<Node2D>) -> Self {
        // Colocando um navio de teste na posição [2, 2] para testar o acerto
        let mut initial_enemy_board = [[0; BOARD_SIZE]; BOARD_SIZE];
        initial_enemy_board[2][2] = 1; // 1 representa um navio

        Self {
            player_board: [[0; BOARD_SIZE]; BOARD_SIZE],
            enemy_board: initial_enemy_board,
            base,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(mouse_event) = event.try_cast::<InputEventMouseButton>() {
            if mouse_event.is_pressed() && mouse_event.get_button_index() == MouseButton::LEFT {
                
                let click_pos = mouse_event.get_global_position();

                if let Some(mut enemy_map) = self.base().try_get_node_as::<TileMapLayer>("CampoIA") {
                    let local_pos = enemy_map.to_local(click_pos);
                    let map_coord = enemy_map.local_to_map(local_pos);

                    if map_coord.x >= 0 && map_coord.x < 10 && map_coord.y >= 0 && map_coord.y < 10 {
                        let x = map_coord.x as usize;
                        let y = map_coord.y as usize;

                        // Verifica o que existe na matriz lógica
                        match self.enemy_board[x][y] {
                            0 => {
                                // ÁGUA: Muda para o tile (8, 3)
                                godot_print!("Errou! Água em [{}, {}]", x, y);
                                self.enemy_board[x][y] = 2; // Marca como "tiro na água" para não repetir
                                
                                enemy_map.set_cell_ex(map_coord)
                                    .source_id(0)
                                    .atlas_coords(Vector2i::new(8, 3))
                                    .done();
                            },
                            1 => {
                                // NAVIO: Muda para o tile (10, 3)
                                godot_print!("FOGO! Navio atingido em [{}, {}]", x, y);
                                self.enemy_board[x][y] = 3; // Marca como "navio atingido"
                                
                                enemy_map.set_cell_ex(map_coord)
                                    .source_id(0)
                                    .atlas_coords(Vector2i::new(10, 3))
                                    .done();
                            },
                            _ => {
                                godot_print!("Você já atirou aqui em [{}, {}]!", x, y);
                            }
                        }
                    }
                } else {
                    godot_print!("Erro: Nó 'CampoIA' não encontrado!");
                }
            }
        }
    }
}