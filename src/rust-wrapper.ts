import { invoke } from '@tauri-apps/api/tauri';

export type Star = {
    x: number,
    y: number,
    currentStar: number
}

export const COLORS: string[] = [
    "#FFFFFF",
    "#fc0202",
    "#fc3c02",
    "#fc7702",
    "#fcb102",
    "#fceb02",
    "#d2fc02",
    "#98fc02",
    "#5efc02",
    "#23fc02",
    "#02fc1b",
    "#02fc55",
    "#02fc90",
    "#02fcca",
    "#02f4fc",
    "#02b9fc",
    "#007efc",
    "#0043fc",
    "#0008fc",
    "#3200fc",
    "#6d00fc",
    "#a800fc",
    "#e300fc",
    "#fc00da",
    "#fc009f",
    "#fc0064",
    "#fc002a"
]


export function load_stars(): Promise<[Star[], string | null]> {
    return invoke("load_stars");
}

export function save_stars(stars: Star[]): Promise<null> {
    return invoke("save_stars", { stars: stars });
}

export function send_stars(jwt: string, stars: Star[]): Promise<null> {
    return invoke("send_stars", { jwt: jwt, stars: stars });
}

export function load_image_colors(): Promise<Star[]> {
    return invoke("load_image_colors");
}