use crate::{MyApp, Work};
use dns_lookup::lookup_host;
use eframe::{
    egui::{self, RichText, Ui},
    epaint::{Color32, FontFamily, FontId},
    CreationContext,
};
use rayon::prelude::*;

use sysinfo::{System, SystemExt};
mod fonts;
use fonts::{setup_custom_fonts, size25, size30};
mod cfg;
use cfg::MOST_COMMON_PORTS_100;

use self::cfg::scan_port_one;
//实现main.rs中定义的结构，定义相关功能实现
impl MyApp {
    //初始化设置及基本数据返回
    pub fn new(cc: &CreationContext) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            host: "192.168.6.1".to_owned(),
            common_port: true,
            all_port: false,
            port_result: Vec::new(),
            domain: "www.baidu.com".to_owned(),
            result: String::new(),
            status: Work::Init,
        }
    }

    //界面首页
    pub fn init(ui: &mut Ui) {
        let mut sys = System::new_all();
        sys.refresh_all();

        ui.heading(size30("欢迎来到黑客小工具"));
        //CPU
        let cpus = sys.cpus().len();
        ui.label(size30(format!("CPU逻辑处理器：{}个", cpus).as_str()));
        let system_name = sys.name();
        let system_kernel_version = sys.kernel_version();
        let system_os_version = sys.os_version();
        let system_host_name = sys.host_name();
        ui.horizontal(|ui| {
            ui.label(size25(
                format!(
                    "系统名：{}",
                    system_name.unwrap_or_else(|| "未找到".to_string())
                )
                .as_str(),
            ));
            ui.label(size25(
                format!(
                    "内核版本：{}",
                    system_kernel_version.unwrap_or_else(|| "未找到".to_string())
                )
                .as_str(),
            ));
            ui.label(size25(
                format!(
                    "系统版本：{}",
                    system_os_version.unwrap_or_else(|| "未找到".to_string())
                )
                .as_str(),
            ));
            ui.label(size25(
                format!(
                    "主机名：{}",
                    system_host_name.unwrap_or_else(|| "未找到".to_string())
                )
                .as_str(),
            ));
        });
        //内存
        let total_memory = sys.total_memory() as f64;
        let used_memory = sys.used_memory() as f64;
        let total_swap = sys.total_swap() as f64;
        let used_swap = sys.used_swap() as f64;
        ui.horizontal(|ui| {
            ui.label(size25(
                format!("总内存：{:.2}G  ", total_memory / 1024.0 / 1024.0 / 1024.0).as_str(),
            ));
            ui.label(size25(
                format!("已使用：{:.2}G    ", used_memory / 1024.0 / 1024.0 / 1024.0).as_str(),
            ));
            ui.label(size25(
                format!("总交换：{:.2}G  ", total_swap / 1024.0 / 1024.0 / 1024.0).as_str(),
            ));
            ui.label(size25(
                format!("已使用：{:.2}G  ", used_swap / 1024.0 / 1024.0 / 1024.0).as_str(),
            ));
        });
        ui.add_sized(
            [50.0, 50.0],
            egui::Hyperlink::from_label_and_url(size30("百度"), "www.baidu.com"),
        );
    }

    //扫描主机端口
    pub fn scan_port(&mut self, ui: &mut Ui) {
        ui.heading(size25("扫描主机端口是否开放"));
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("主机IP: ")
                    .font(FontId::proportional(30.0))
                    .color(Color32::YELLOW),
            );

            ui.add_sized(
                [200.0, 30.0],
                egui::TextEdit::singleline(&mut self.host)
                    .font(FontId::new(25.0, FontFamily::Monospace)),
            );

            //单选框
            match self.common_port {
                true => {
                    if ui.radio(self.common_port, "常见端口").clicked() {
                        self.common_port = false;
                        self.all_port = true;
                    }
                    if ui.radio(self.all_port, "全端口").clicked() {
                        self.common_port = false;
                        self.all_port = true;
                    }
                }
                false => {
                    if ui.radio(self.common_port, "常见端口").clicked() {
                        self.common_port = true;
                        self.all_port = false;
                    }
                    if ui.radio(self.all_port, "全端口").clicked() {
                        self.common_port = true;
                        self.all_port = false;
                    }
                }
            }

            if ui.button(size25("扫描").color(Color32::RED)).clicked() {
                self.result = String::new();
                if self.common_port {
                    self.scan_ports(self.host.clone(), true);
                } else {
                    self.scan_ports(self.host.clone(), false);
                }

                //self.result = self.host.clone() + "Ok";
            }
        });
        // ui.colored_label(Color32::WHITE, size25(&self.result));
        for i in self.port_result.iter() {
            if i != &0u16 {
                self.result.push_str(&(format!("{}端口已开放!\n", i)));
            }
        }
        ui.add_sized(
            [500.0, 300.0],
            egui::TextEdit::multiline(&mut self.result)
                .font(FontId::new(25.0, FontFamily::Monospace)),
        );
        //ui.label(format!("comm{},all{}",self.common_port,self.all_port));调试
    }

    pub fn scan_ports(&mut self, ip: String, common: bool) {
        if common {
            let mut open_port = MOST_COMMON_PORTS_100
                .into_par_iter()
                .map(|port| scan_port_one(ip.clone(), *port))
                .collect::<Vec<u16>>();
            open_port.sort_unstable();
            open_port.dedup();
            self.port_result = open_port;
        } else {
            let mut open_port = [1u16..=65535u16]
                .into_par_iter()
                .map(|mut port| scan_port_one(ip.clone(), port.next().unwrap()))
                .collect::<Vec<u16>>();
            open_port.sort_unstable();
            open_port.dedup();
            self.port_result = open_port;
        }
    }

    //域名查IP
    pub fn domain_to_ip(&mut self, ui: &mut Ui) {
        ui.heading(size25("根据域名查询IP"));
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("域名: ")
                    .font(FontId::proportional(30.0))
                    .color(Color32::YELLOW),
            );

            ui.add_sized(
                [300.0, 30.0],
                egui::TextEdit::singleline(&mut self.domain)
                    .font(FontId::new(25.0, FontFamily::Monospace)),
            );
            if ui.button(size25("查询").color(Color32::RED)).clicked() {
                let hosts = lookup_host(&self.domain);
                let ok_hosts;
                match hosts {
                    Ok(h) => {
                        ok_hosts = h;
                        let mut res = format!("{}解析出来的IP有:\n", self.domain);
                        for i in ok_hosts {
                            res.push_str(&(i.to_string() + "\n"));
                        }
                        println!("{}", res);
                        self.result = res;
                    }
                    Err(_) => self.result = "解析IP失败".to_owned(),
                }
            }
        });
        ui.add_sized(
            [500.0, 300.0],
            egui::TextEdit::multiline(&mut self.result)
                .font(FontId::new(25.0, FontFamily::Monospace)),
        );
    }
}
