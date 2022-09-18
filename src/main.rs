#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, menu, FontId, RichText};
use eframe::epaint::{Vec2, Color32};

mod myapp;


//主函数启动gui
fn main() {
    //let _options = eframe::NativeOptions::default();
    let options2 = eframe::NativeOptions {
        initial_window_size: Option::from(Vec2::new(800_f32, 600_f32)),
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        initial_window_pos: None,
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        run_and_return: false,
        ..Default::default() //icon和硬件加速没设置
    };

    eframe::run_native(
        "彪神的黑客工具箱",
        options2,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}


//界面使用的功能
#[derive(Debug)]
enum Work {
    //初始化界面显示
    Init,
    //扫描主机端口
    ScanPort,
    //域名查IP
    DomainToIP,
}

//界面所需的运行参数及功能实现
#[derive(Debug)]
struct MyApp {
    //待扫描的主机
    host: String,
    common_port:bool,
    all_port:bool,
    port_result:Vec<u16>,
    //域名查IP
    domain: String,

    //作为运行结果显示
    result: String,
    //用户执行的操作
    status: Work,
}

//gui实时刷新
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| -> egui::Response {
            // use egui::menu;
            menu::bar(ui, |ui| {
                ui.menu_button(
                    RichText::new("信息收集").font(FontId::proportional(30.0)).color(Color32::RED),
                    |ui| {
                        if ui
                            .button(RichText::new("扫描主机端口").font(FontId::proportional(25.0)))
                            .clicked()
                        {
                            self.status = Work::ScanPort;
                        }
                        if ui
                            .button(RichText::new("域名查IP").font(FontId::proportional(25.0)))
                            .clicked()
                        {
                            self.status = Work::DomainToIP;
                        }
                    },
                );
            });

            //相关功能在myapp.rs中实现
            match self.status {
                //初始化
                Work::Init => {
                    MyApp::init(ui);
                    ui.label("")
                }
                //扫描主机端口
                Work::ScanPort => {
                    self.scan_port(ui);
                    ui.label("")
                }
                //域名查IP
                Work::DomainToIP => {
                    self.domain_to_ip(ui);
                    ui.label("")
                }
            }

            //ui.label(format!("Result: {}", self.result));
            // ui.colored_label(
            //     Color32::WHITE,
            //     RichText::new(&self.result).font(FontId::proportional(20.0)),
            // );
        });
    }
}
