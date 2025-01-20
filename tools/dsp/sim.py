"""
Some simulation tools for playing with signals.
"""

import numpy as np
from numpy.typing import NDArray
import math

SAMPLE_FREQ = 44100

def gen_tone(
    duration: float,
    freq: float,
    mag: float,
    sample_freq: float = SAMPLE_FREQ,
) -> NDArray:
    """
    Generate a tone of the provided duration/freq/mag

    Args:
        duration: The duration of the simulated tone (in seconds)
        freq: The frequency of the simulated signal.
        mag: The magnitude of the simulated signal.
        sample_freq: The sample frequency.
    """
    t = np.linspace(0, duration, int(duration * sample_freq))
    return mag * np.sin(2 * math.pi * freq * t)


def gen_noise(
    duration: float,
    std_dev: float = 1.0,
    sample_freq: float = SAMPLE_FREQ,
) -> NDArray:
    """
    Generate some noise centered around 0.

    Args:
        duration: The duration
        std_dev: The standard deviation of the noise.
        sample_freq: The sample frequency.
    """
    return np.random.normal(0, std_dev, int(duration * sample_freq))


def to_f32(sig: NDArray) -> NDArray[np.float32]:
    """
    Convert a np.array to float32.

    Args:
        sig: The signal.
    """
    return np.float32(sig)


def filter(signal: NDArray, a: list[float], b: list[float]) -> NDArray:
    """
    Filter a signal.

    Args:
        signal: The signal to filter.
        a: The filter denominator coefficients.
        b: The filter numerator coefficients.

    Returns the filtered signals.
    """
    filtered = np.zeros(len(signal))
    for m in range(len(b), len(signal)):
        filtered[m] = b[0] * signal[m]
        for i in range(1, len(b)):
            filtered[m] += a[i] * filtered[m-i] + b[i] * signal[m - i]

    return filtered


def filter2(signal: NDArray, a: list[float], b: list[float], dtype=np.float32) -> NDArray:
    """
    Filter a signal.

    Args:
        signal: The signal to filter.
        a: The filter denominator coefficients.
        b: The filter numerator coefficients.

    Returns the filtered signals.
    """
    order = len(b)

    # Bootstrap the history ring buffer
    in_history = np.zeros(order, dtype=dtype)
    out_history = np.zeros(order, dtype=dtype)
    ring_index = 0

    filtered = np.zeros(len(signal), dtype=dtype)
    for m in range(0, len(signal)):
        # First term
        filtered[m] = b[0] * signal[m]

        # Next N-1 terms
        for i in range(1, order):
            prev = ring_index - i
            if prev < 0:
                prev += order

            filtered[m] += a[i] * out_history[prev] + b[i] * in_history[prev]

        in_history[ring_index] = signal[m]
        out_history[ring_index] = filtered[m]
        ring_index = (ring_index + 1) % order

    return filtered
