use bevy_ecs::prelude::*;
use macroquad::prelude::*;

// ===== ECS 组件 =====
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

// ===== ECS 系统 =====

// 移动系统：position += velocity
fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query {
        pos.x += vel.x * get_frame_time();
        pos.y += vel.y * get_frame_time();
    }
}

// 渲染系统：使用 macroquad 绘制实体
fn render_system(query: Query<&Position>) {
    clear_background(BLACK);

    for pos in &query {
        draw_circle(pos.x, pos.y, 20.0, YELLOW);
    }
}

#[macroquad::main("Macroquad + ECS")]
async fn main() {
    // ===== 初始化 ECS =====
    let mut world = World::new();
    let mut schedule = Schedule::default();

    // 系统添加进 schedule
    schedule.add_systems((movement_system,));

    // 实体
    world.spawn((
        Position { x: 100., y: 100. },
        Velocity { x: 50., y: 20. },
    ));

    loop {
        // ===== 运行 ECS 系统 =====
        schedule.run(&mut world);

        // ===== 渲染（非 ECS，也可写成 ECS 系统）=====
        {
            let mut query = world.query::<&Position>();
            clear_background(BLACK);
            for pos in query.iter(&world) {
                draw_circle(pos.x, pos.y, 20.0, GREEN);
            }
        }

        next_frame().await;
    }
}
