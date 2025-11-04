type SimulationFieldKind = "Slider" | "ReadOnly";

interface SimulationField {
  kind: SimulationFieldKind;
  max: number;
  min: number;
  value: number;
}
