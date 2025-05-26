pub const PI : f32 = 3.14159265358979323;
const PIXEL_FREQ : f32 = 12.5875000000; //PIXEL_FREQUENCY in MHZ
const NYQUIST_FREQ : f32 = 6.2937500000; //Nyquist Frequency in MHZ;
const Y_FREQ_MAX : f32 = 4.200000000; //Maximum Y Freqeuncy in MHZ
const I_FREQ_MAX : f32 = 1.30000000; //Maximum I Frequency on transmitter side in MHZ
const Q_FREQ_MAX : f32 = 0.60000000; //Maximum Q frequency on transmitter side in MHZ
const UV_FREQ_MAX : f32 = 0.60000000; //Maximum UV frequency on transmitter side in MHZ
//pub fn rgb_to_yuv(r : f32, g : f32, b : f32) -> (f32, f32, f32) {
//}
//Most TVs used UV decoding instead of iq decoding
//https://forums.nesdev.org/viewtopic.php?t=7261&start=45
//Shader used to emulated the effects of NTSC video coding and decoding.
//This emulator uses an interlaced version version of VGA resolution.
//Calculated using http://t-filter.engineerjs.com/
pub const Y_FILTER : [f32; 19] = [0.0329496349929555,
    0.08394084243079884,
    0.0313875395357978,
    -0.05507199884540697,
    0.0036253651287004272,
    0.07295682180623113,
    -0.06575019693219097,
    -0.08809058165218961,
    0.303679835097292,
    0.5942133818524733,
    0.303679835097292,
    -0.08809058165218961,
    -0.06575019693219097,
    0.07295682180623113,
    0.0036253651287004272,
    -0.05507199884540697,
    0.0313875395357978,
    0.08394084243079884,
    0.0329496349929555
];
const Y_DELAY : u8 = 9;
const Y_FILTER_FIXED: [i16; 19] = [1080,
    2751,
    1029,
    -1805,
    119,
    2391,
    -2155,
    -2887,
    9951,
    19471,
    9951,
    -2887,
    -2155,
    2391,
    119,
    -1805,
    1029,
    2751,
    1080
]; 
