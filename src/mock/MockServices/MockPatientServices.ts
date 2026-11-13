import type { Patient } from "../../objects/Patient";

// src/services/MockPatientService.ts
export class MockPatientService {
  private static patients: Patient[] = [
    {
      id: 1,
      firstName: "Carlos",
      lastName: "Mendoza",
      name: "Carlos Mendoza",
      age: 28,
      email: "carlos.mendoza@example.com",
      phone: "+52 555 123 4567"
    },
    {
      id: 2,
      firstName: "Ana",
      lastName: "García",
      name: "Ana García",
      age: 34,
      email: "ana.garcia@example.com",
      phone: "+52 555 987 6543"
    },
    {
      id: 3,
      firstName: "Luis",
      lastName: "Rodríguez",
      name: "Luis Rodríguez",
      age: 45,
      email: "luis.rodriguez@example.com"
    },
    {
      id: 4,
      firstName: "Sofía",
      lastName: "López",
      name: "Sofía López",
      age: 22
    }
  ];

  public static getAll(): Patient[] {
    return this.patients;
  }

  public static getById(id: string): Patient | undefined {
    return this.patients.find(p => p.id === id);
  }
}