export type GameType = 'Arrastre' | 'Tap' | 'Puzzle';

export interface Game {
    id: string;
    name: string;
    description: string;
    type: string;
    iconURL?: string;
    imageURL?: string;
}