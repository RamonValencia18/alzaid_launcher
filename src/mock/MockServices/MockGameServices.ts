import type { Game } from "../../objects/Game";

// src/services/MockGameService.ts
export class MockGameService {
  private static games: Game[] = [
    {
      id: "1",
      name: "Juego del Cerdito",
      description: "Arma un rompecabezas de cerdito.",
      type: "Arrastre"
    },
    {
      id: "2",
      name: "Juego de los Topos",
      description: "Golpea los topos que van saliendo de la superficie.",
      type: "Tap"
    },
    {
      id: "3",
      name: "Figuras",
      description: "Arrastra la figura al area correspondiente.",
      type: "Arrastre"
    },
    {
      id: "4",
      name: "Tap",
      description: "Presiona la figura que salga en la pantalla.",
      type: "Tap"
    }
  ];

  public static getAll(): Game[] {
    return this.games;
  }

  public static getById(id: string): Game | undefined {
    return this.games.find(g => g.id === id);
  }
}