use super::Tile;

mod binary_test;
mod backtracker_test;
mod prim_test;
mod test_utils;

#[test]
fn tile_equality()
{
    let empty_tile = Tile::Empty;
    let full_tile = Tile::new();
    let partial_tile = Tile::Connected {
        up: true,
        down: true,
        left: false,
        right: false,
    };

    assert!(empty_tile != full_tile,
            "Empty tile tested equal to full tile");
    assert!(empty_tile != partial_tile,
            "Empty tile tested equal to partialy full tile");
    assert!(partial_tile != full_tile,
            "Partially full tile tested equal to full tile");
    assert!(partial_tile == partial_tile,
            "Tile tested inequal to itself");
}
