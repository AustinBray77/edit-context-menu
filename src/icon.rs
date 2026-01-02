use egui::ColorImage;
use std::error::Error;
use std::fs;
use windows::Win32::Graphics::Gdi::BITMAPINFO;
use windows::Win32::Graphics::Gdi::BITMAPINFOHEADER;
use windows::Win32::Graphics::Gdi::CreateCompatibleDC;
use windows::Win32::Graphics::Gdi::DIB_RGB_COLORS;
use windows::Win32::Graphics::Gdi::DeleteDC;
use windows::Win32::Graphics::Gdi::DeleteObject;
use windows::Win32::Graphics::Gdi::GetDIBits;
use windows::Win32::Graphics::Gdi::SelectObject;
use windows::Win32::UI::Shell::ExtractIconExW;
use windows::Win32::UI::WindowsAndMessaging::DestroyIcon;
use windows::Win32::UI::WindowsAndMessaging::GetIconInfoExW;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::ICONINFOEXW;
use windows::core::PCWSTR;

use crate::edit_context_lib::addtopath::check_in_path;

pub fn get_images_from_exe<'a>(executable_path: &'a str) -> Result<ColorImage, Box<dyn Error>> {
    let path: Box<str> = match fs::exists(executable_path) {
        Ok(true) => executable_path.into(),
        _ => check_in_path(executable_path)?,
    };

    unsafe {
        let path_cstr = path
            .encode_utf16()
            .chain("\0".encode_utf16())
            .collect::<Vec<u16>>();
        let path_pcwstr = PCWSTR(path_cstr.as_ptr());
        let num_icons_total = ExtractIconExW(path_pcwstr, -1, None, None, 0);

        if num_icons_total == 0 {
            return Err(format!("No icons found for {}", path).into()); // No icons extracted
        }

        let mut icon = vec![HICON::default(); 1];

        let num_icons_fetched = ExtractIconExW(path_pcwstr, 0, Some(icon.as_mut_ptr()), None, 1);

        if num_icons_fetched == 0 {
            return Err(format!("No icons fetched for {}", path).into()); // No icons extracted
        }

        let image = convert_hicon_to_rgba_image(icon[0])?;

        Ok(image)
    }
}

pub fn convert_hicon_to_rgba_image<'a>(hicon: HICON) -> Result<ColorImage, Box<dyn Error>> {
    unsafe {
        let mut icon_info = ICONINFOEXW::default();
        icon_info.cbSize = std::mem::size_of::<ICONINFOEXW>() as u32;

        if !GetIconInfoExW(hicon, &mut icon_info).as_bool() {
            return Err(format!(
                "icon • GetIconInfoExW: {} {}:{}",
                file!(),
                line!(),
                column!()
            )
            .into());
        }
        let hdc_screen = CreateCompatibleDC(None);
        let hdc_mem = CreateCompatibleDC(Some(hdc_screen));
        let hbm_old = SelectObject(hdc_mem, icon_info.hbmColor.into());

        let mut bmp_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: icon_info.xHotspot as i32 * 2,
                biHeight: -(icon_info.yHotspot as i32 * 2),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: DIB_RGB_COLORS.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut buffer: Vec<u8> =
            vec![0; (icon_info.xHotspot * 2 * icon_info.yHotspot * 2 * 4) as usize];

        if GetDIBits(
            hdc_mem,
            icon_info.hbmColor,
            0,
            icon_info.yHotspot * 2,
            Some(buffer.as_mut_ptr() as *mut _),
            &mut bmp_info,
            DIB_RGB_COLORS,
        ) == 0
        {
            return Err(format!("GetDIBits: {} {}:{}", file!(), line!(), column!()).into());
        }

        // Clean up
        SelectObject(hdc_mem, hbm_old);
        if !DeleteDC(hdc_mem).as_bool() {
            return Err("Failed to delete DC".into());
        }
        if !DeleteDC(hdc_screen).as_bool() {
            return Err("Failed to delete DC".into());
        }
        if !DeleteObject(icon_info.hbmColor.into()).as_bool() {
            return Err("Failed to delete object".into());
        }
        if !DeleteObject(icon_info.hbmMask.into()).as_bool() {
            return Err("Failed to delete object".into());
        }
        DestroyIcon(hicon)?;

        //bgra_to_rgba(buffer.as_mut_slice());

        let rgba = bgra_to_rgba(buffer);

        //let image = ImageBuffer::from_raw(icon_info.xHotspot * 2, icon_info.yHotspot * 2, buffer)
        //.ok_or_else(|| Error::ImageContainerNotBigEnough)?;

        let color_image = ColorImage::from_rgba_unmultiplied(
            [
                icon_info.xHotspot as usize * 2,
                icon_info.yHotspot as usize * 2,
            ],
            &rgba,
        );

        Ok(color_image)
    }
}

fn bgra_to_rgba(bgra: Vec<u8>) -> Vec<u8> {
    // Expect length to be multiple of 4 (B,G,R,A per pixel). If not, extra bytes copied as-is.
    let mut out = Vec::with_capacity(bgra.len());
    let mut i = 0;
    let len = bgra.len();
    while i + 4 <= len {
        // bgra[i+0]=B, i+1=G, i+2=R, i+3=A => push R,G,B,A
        out.push(bgra[i + 2]);
        out.push(bgra[i + 1]);
        out.push(bgra[i + 0]);
        out.push(bgra[i + 3]);
        i += 4;
    }
    // Copy any trailing bytes (shouldn't occur for well-formed image data)
    if i < len {
        out.extend_from_slice(&bgra[i..]);
    }
    out
}
