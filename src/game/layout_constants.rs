use util::Vector2D;

pub const SCREEN_SIZE: Vector2D<i32> = Vector2D { x: 120, y: 52 };

pub const LOGO_POSITION: Vector2D<i32> = Vector2D { x: 0, y: 0 };
pub const LOGO_SIZE: Vector2D<i32> = Vector2D { x: 45, y: 6 };

pub const GAME_AREA_POSITION: Vector2D<i32> = Vector2D {
    x: 0,
    y: LOGO_SIZE.y + 1,
};
pub const GAME_AREA_SIZE: Vector2D<i32> = Vector2D {
    x: SCREEN_SIZE.x - (CONSOLE_SIZE.x + 2),
    y: SCREEN_SIZE.y - (LOGO_SIZE.y + 1),
};
pub const GAME_AREA_CENTRE: Vector2D<i32> = Vector2D {
    x: GAME_AREA_SIZE.x / 2,
    y: GAME_AREA_SIZE.y / 2,
};

pub const INPUT_FIELD_POSITION: Vector2D<i32> = Vector2D {
    x: LOGO_SIZE.x + 2,
    y: 0,
};
pub const INPUT_FIELD_SIZE: Vector2D<i32> = Vector2D {
    x: SCREEN_SIZE.x - (LOGO_SIZE.x + 2) - (CONSOLE_SIZE.x + 2),
    y: LOGO_SIZE.y,
};

pub const CONSOLE_POSITION: Vector2D<i32> = Vector2D {
    x: SCREEN_SIZE.x - CONSOLE_SIZE.x,
    y: 0,
};
pub const CONSOLE_SIZE: Vector2D<i32> = Vector2D {
    x: 32,
    y: SCREEN_SIZE.y,
};
