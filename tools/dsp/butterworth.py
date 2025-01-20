from scipy import signal
import numpy as np
from numpy.typing import NDArray

from tools.dsp.sim import SAMPLE_FREQ


def generate_filter(
    cutoff_frequency: float,
    order: int,
    sample_freq: float,
    dtype=np.float32
) -> tuple[NDArray, NDArray]:
    """
    Generate a filter low-pass Butterworth filter.

    Args:
        cutoff_frequency the cutoff frequency.
        order: The filter order.
        sample_freq: The sampling frequency.

    Returns: The discrete filter (denominator, numerator) coefficients.
    """
    # Butterworth filter
    n = order

    # wc term
    wc = 2 * np.pi * cutoff_frequency

    # Calculate filter coefficients
    # https://en.wikipedia.org/wiki/Butterworth_filter#Normalized_Butterworth_polynomials
    a = np.zeros(n + 1)
    gamma = np.pi / (2.0 * n)
    a[0] = 1
    for k in range(0, n):
        a[k + 1] = a[k] * np.cos(k * gamma) / np.sin((k + 1) * gamma)

    # Adjust the coefficients for the filter frequency.
    c = np.zeros(n + 1)
    for k in range(0, n + 1):
        c[n - k] = a[k] / pow(wc, k)

    # Express the coefficients above in terms of a normalized numerator/denominator
    # for a continuous (analog) transfer function.
    low_pass = signal.TransferFunction([1], c)

    # Compute the discrete low pass using a bilinear transformation with alpha=0.5
    # https://en.wikipedia.org/wiki/Bilinear_transform#Transformation_of_a_General_LTI_System
    dt = 1.0 / sample_freq
    discrete_low_pass = low_pass.to_discrete(dt, method='gbt', alpha=0.5)
    a = -discrete_low_pass.den
    b = discrete_low_pass.num

    return dtype(a), dtype(b)
