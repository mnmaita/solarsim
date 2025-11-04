import { render, screen } from "@testing-library/react";
import Home from "../page";
import { usePolling } from "../hooks/usePolling";
import {
  mockSimulationReadOnlyField,
  mockSimulationSliderField,
  networkError,
} from "../__mocks__/constants";

jest.mock("../hooks/usePolling");

describe("Home", () => {
  it("renders server unavailable error", () => {
    (usePolling as jest.Mock).mockReturnValue({
      data: networkError,
      loading: false,
      error: new Error(""),
      pause: () => {},
      resume: () => {},
    });

    render(<Home />);

    expect(screen.getByText("Server is unavailable :(")).toBeInTheDocument();
  });

  it("renders simulation fields", () => {
    (usePolling as jest.Mock).mockReturnValue({
      data: {
        slider_field: mockSimulationSliderField,
        read_only_field: mockSimulationReadOnlyField,
      },
      loading: false,
      error: null,
      pause: () => {},
      resume: () => {},
    });

    render(<Home />);

    expect(screen.getByLabelText("Slider Field")).toBeInTheDocument();

    expect(screen.getByLabelText("Read Only Field")).toBeInTheDocument();
  });

  it("renders Home page unchanged", () => {
    (usePolling as jest.Mock).mockReturnValue({
      data: {
        slider_field: mockSimulationSliderField,
        read_only_field: mockSimulationReadOnlyField,
      },
      loading: false,
      error: null,
      pause: () => {},
      resume: () => {},
    });

    const { container } = render(<Home />);

    expect(container).toMatchSnapshot();
  });
});
