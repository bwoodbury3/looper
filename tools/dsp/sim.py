"""
Some simulation tools for playing with signals.
"""

import numpy as np
import math

SAMPLE_FREQ = 44100

def gen_tone(
    duration: float,
    freq: float,
    mag: float,
    sample_freq: float = SAMPLE_FREQ,
) -> np.array:
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
) -> np.array:
    """
    Generate some noise centered around 0.

    Args:
        duration: The duration
        std_dev: The standard deviation of the noise.
        sample_freq: The sample frequency.
    """
    return np.random.normal(0, std_dev, int(duration * sample_freq))


def filter(signal: np.array, a: list[float], b: list[float]) -> np.array:
    """
    Filter a signal.

    Args:
        signal: The signal to filter.
        a: The filter denominator coefficients.
        b: The filter numerator coefficients.

    Returns the filtered signals.
    """
    filtered = np.zeros(len(signal))
    for m in range(3, len(signal)):
        filtered[m] = b[0] * signal[m]
        for i in range(1, len(b)):
            filtered[m] += a[i] * filtered[m-i] + b[i] * signal[m - i]

    return filtered