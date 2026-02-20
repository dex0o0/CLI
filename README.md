# CLI (dex)

`dex` یک ابزار خط فرمان (CLI) است که با Rust نوشته شده و برای کارهای روزمره سیستم طراحی شده است.

## Features

- نمایش وضعیت سیستم در قالب TUI با به‌روزرسانی دوره‌ای
- مدیریت Wi-Fi با استفاده از `nmcli`
- باز کردن سرویس‌های وب (GitHub, Gmail, YouTube Music)
- ارسال نوتیفیکیشن با اسکریپت محلی
- دانلود فایل از URL یا لیست URL داخل فایل

## Requirements

- Linux
- Rust toolchain
- `nmcli` (برای دستورات Wi-Fi)

## Install

```bash
cd /path/to/cloned/repo
chmod +x install.sh
./install.sh
```

## Usage

```bash
dex --help
dex --version
```

### Wi-Fi commands

```bash
dex wifi list
dex wifi connect <NETWORK_NAME>
dex wifi connection
dex wifi disconnect <NETWORK_DEVICE>
```

### System status

```bash
dex status
```

در حالت `status` یک رابط TUI باز می‌شود که اطلاعات سیستم را هر ۲ ثانیه به‌روزرسانی می‌کند.
برای خروج از این صفحه از `q` یا `Esc` استفاده کنید.

### Download

```bash
dex dl --url <URL>
dex dl --url <URL> --filename <FILE_NAME>
dex dl --file <PATH_TO_TEXT_FILE>
```

## Project structure

- `src/main.rs`: ورودی برنامه و تعریف commandها
- `src/commands/tui.rs`: رابط TUI برای وضعیت سیستم
- `src/commands/scan_sys.rs`: جمع‌آوری اطلاعات سخت‌افزاری
- `src/commands/command.rs`: سایر commandهای سیستم

## License

MIT
