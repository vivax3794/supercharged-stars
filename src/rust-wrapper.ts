import { invoke } from '@tauri-apps/api/tauri';

export type Star = {
    x: number,
    y: number,
    color: number
}

export const COLORS: string[] = [
    "#f73693",
    "#4392a0",
    "#1172b2",
    "#edd45a",
    "#b403a8",
    "#7e15af",
    "#ba2362",
    "#06aa19",
    "#aaf96d",
    "#2a8791",
    "#5ad117",
    "#bf8209",
    "#099b88",
    "#ac1804",
    "#52ce61",
    "#4dcc68",
    "#9a43a5",
    "#1250b5",
    "#a0b709",
    "#d0ed12",
    "#8e1531",
    "#2daeb5",
    "#24ad38",
    "#df33b0",
    "#fce54e",
]


export function load_stars(): Promise<Star[]> {
    return invoke("load_stars");
}