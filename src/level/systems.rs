// fn init_map(mut commands: Commands, asset: Res<AssetServer>) {
//     let handle: Handle<Image> = asset.load("tiles.png");
//     let map_size = TilemapSize::new(50, 50);
//     let tilemap_entity = commands.spawn_empty().id();
//     let mut tile_storage = TileStorage::empty(map_size);

//     for x in 0..map_size.x {
//         for y in 0..map_size.y {
//             let tile_pos = TilePos { x, y };
//             let tile_entity = commands
//                 .spawn(TileBundle {
//                     position: tile_pos,
//                     tilemap_id: TilemapId(tilemap_entity),
//                     ..Default::default()
//                 })
//                 .id();
//             tile_storage.set(&tile_pos, tile_entity);
//         }
//     }

//     let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
//     let grid_size = tile_size.into();
//     let map_type = TilemapType::default();

//     commands.entity(tilemap_entity).insert(TilemapBundle {
//         grid_size,
//         map_type,
//         size: map_size,
//         storage: tile_storage,
//         texture: TilemapTexture::Single(handle),
//         tile_size,
//         anchor: TilemapAnchor::Center,
//         ..Default::default()
//     });
// }

// fn load_tmx(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let map_handle = crate::helpers::tiled::TiledMapHandle(asset_server.load("map.tmx"));

//     commands.spawn(crate::helpers::tiled::TiledMapBundle {
//         tiled_map: map_handle,
//         ..Default::default()
//     });
// }
