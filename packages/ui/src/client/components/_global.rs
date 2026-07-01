use dioxus::prelude::*;
use dioxus_radio::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Task{
    id: usize,
    title: String,
    assignee: String,
    progress: u8
}

struct KanbanState {
    pub board_name: String,
    pub tasks: HashMap<usize, Task>,
    pub task_ids: Vec<usize>
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum KanbanChannel {
    BoardMeta,
    TaskListStructure,
    TaskDetail(usize),
}

impl RadioChannel<KanbanState> for KanbanChannel {}

pub fn App() -> Element {
    // Khởi tạo trạng thái ban đầu với 2 task mẫu
    use_init_radio_station::<KanbanState, KanbanChannel>(|| {
        let mut tasks = HashMap::new();
        tasks.insert(101, Task { id: 101, title: "Thiết kế DB".to_string(), assignee: "An".to_string(), progress: 10 });
        tasks.insert(102, Task { id: 102, title: "Viết API Auth".to_string(), assignee: "Bình".to_string(), progress: 40 });

        KanbanState {
            board_name: "Dự án Alpha".to_string(),
            tasks,
            task_ids: vec![101, 102],
        }
    });

    // --- ADVANCED: GIẢ LẬP WEBSOCKET ĐẨY DỮ LIỆU NGẦM ---
    // Ta lấy một radio thuộc kênh bất kỳ (hoặc tạo một worker độc lập) để ghi dữ liệu về
    let mut background_radio = use_radio::<KanbanState, KanbanChannel>(KanbanChannel::BoardMeta);
    
    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;
            
            // Giả lập Server báo: "Task 101 vừa tăng tiến độ!"
            let mut state = background_radio.write_to_channel(KanbanChannel::TaskDetail(101));
            if let Some(task) = state.tasks.get_mut(&101) {
                if task.progress < 100 {
                    task.progress += 10;
                    println!("📡 [WS Server] Đã cập nhật tự động Task 101 lên {}%", task.progress);
                }
            }
        }
    });

    println!("[Render] Toàn bộ Bảng Kanban vừa render lại!");

    rsx! {
        div { style: "max-width: 600px; margin: 30px auto; padding: 20px; background: #f7fafc; border-radius: 12px; box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1); font-family: system-ui;",
            BoardHeader {}
            TaskList {}
            AddTaskButton {}
        }
    }
}

// ==========================================
// 4. CÁC COMPONENT CON CHUYÊN BIỆT
// ==========================================

// --- COMPONENT TIÊU ĐỀ (Chỉ nghe kênh BoardMeta) ---
#[component]
fn BoardHeader() -> Element {
    let mut radio = use_radio::<KanbanState, KanbanChannel>(KanbanChannel::BoardMeta);
    println!("✏️ [Render] Tiêu đề Bảng vừa render lại!");

    rsx! {
        div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
            h2 { "📋 {radio.read().board_name}" }
            button {
                onclick: move |_| {
                    radio.write().board_name = "Dự án Beta (Đã đổi tên)".to_string();
                },
                "Đổi tên dự án"
            }
        }
    }
}

// --- COMPONENT DANH SÁCH (Chỉ nghe khi CẤU TRÚC danh sách thay đổi) ---
#[component]
fn TaskList() -> Element {
    let radio = use_radio::<KanbanState, KanbanChannel>(KanbanChannel::TaskListStructure);
    println!("📦 [Render] Khung Danh sách Task vừa render lại!");

    // Đọc danh sách ID hiện tại
    let ids = radio.read().task_ids.clone();

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 12px;",
            for id in ids {
                // Truyền ID vào Component con. Bản thân Component con sẽ tự lo việc kết nối Radio.
                TaskItem { key: "{id}", task_id: id }
            }
        }
    }
}

// --- COMPONENT ITEM CÔNG VIỆC (Mỗi item nghe 1 kênh DỘNG riêng) ---
#[component]
fn TaskItem(task_id: usize) -> Element {
    // 🔥 ĐẲNG CẤP Ở ĐÂY: Đăng ký chính xác kênh TaskDetail(task_id)
    let mut radio = use_radio::<KanbanState, KanbanChannel>(KanbanChannel::TaskDetail(task_id));
    
    println!("🎯 [Render] CHỈ CÓ Task #{task_id} render lại!");

    // Đọc dữ liệu của chính mình từ State tổng
    let state = radio.read();
    let task = match state.tasks.get(&task_id) {
        Some(t) => t,
        None => return rsx! {
            div { "Task không tồn tại" }
        }
    };

    rsx! {
        div { style: "background: white; padding: 15px; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.05); border-left: 5px solid #4c51bf;",
            div { style: "display: flex; justify-content: space-between;",
                span { style: "font-weight: bold; color: #2d3748;", "{task.title}" }
                span { style: "font-size: 12px; background: #edf2f7; padding: 2px 6px; border-radius: 4px;",
                    "👤 {task.assignee}"
                }
            }
            // Thanh hiển thị tiến độ
            div { style: "margin-top: 10px; background: #e2e8f0; height: 8px; border-radius: 4px; overflow: hidden;",
                div { style: "background: #4c51bf; height: 100%; width: {task.progress}%; transition: width 0.3s;" }
            }
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-top: 8px;",
                span { style: "font-size: 12px; color: #718096;", "Tiến độ: {task.progress}%" }
                button {
                    style: "font-size: 11px; padding: 2px 8px;",
                    onclick: move |_| {
                        // Tự tăng tiến độ bằng tay
                        if radio.read().tasks.get(&task_id).unwrap().progress < 100 {
                            radio.write().tasks.get_mut(&task_id).unwrap().progress += 5;
                        }
                    },
                    "＋ Tiến độ"
                }
            }
        }
    }
}

// --- COMPONENT NÚT THÊM TASK MỚI (Bắn tín hiệu làm thay đổi cấu trúc danh sách) ---
#[component]
fn AddTaskButton() -> Element {
    // Cần quyền ghi vào cấu trúc danh sách
    let mut radio = use_radio::<KanbanState, KanbanChannel>(KanbanChannel::TaskListStructure);

    rsx! {
        button {
            style: "width: 100%; margin-top: 15px; padding: 10px; background: #4c51bf; color: white; border: none; border-radius: 6px; font-weight: bold; cursor: pointer;",
            onclick: move |_| {
                let new_id = rand::random::<usize>() % 1000; // Giả lập tạo ID ngẫu nhiên

                let mut state = radio.write();
                state
                    .tasks
                    .insert(
                        new_id,
                        Task {
                            id: new_id,
                            title: format!("Task mới #{new_id}"),
                            assignee: "Thành viên mới".to_string(),
                            progress: 0,
                        },
                    );
                state.task_ids.push(new_id);
            },
            "➕ Thêm công việc mới"
        }
    }
}