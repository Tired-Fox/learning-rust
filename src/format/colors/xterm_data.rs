use phf::{phf_map, Map};

/// XTERM codes and their corresponding rgb values.
pub const CODE_TO_RGB: Map<u8, (u8, u8, u8)> = phf_map! {
    0u8 => (0, 0, 0),
    1u8 => (128, 0, 0),
    2u8 => (0, 128, 0),
    3u8 => (128, 128, 0),
    4u8 => (0, 0, 128),
    5u8 => (128, 0, 128),
    6u8 => (0, 128, 128),
    7u8 => (192, 192, 192),
    8u8 => (128, 128, 128),
    9u8 => (255, 0, 0),
    10u8 => (0, 255, 0),
    11u8 => (255, 255, 0),
    12u8 => (0, 0, 255),
    13u8 => (255, 0, 255),
    14u8 => (0, 255, 255),
    15u8 => (255, 255, 255),
    16u8 => (0, 0, 0),
    17u8 => (0, 0, 95),
    18u8 => (0, 0, 135),
    19u8 => (0, 0, 175),
    20u8 => (0, 0, 215),
    21u8 => (0, 0, 255),
    22u8 => (0, 95, 0),
    23u8 => (0, 95, 95),
    24u8 => (0, 95, 135),
    25u8 => (0, 95, 175),
    26u8 => (0, 95, 215),
    27u8 => (0, 95, 255),
    28u8 => (0, 135, 0),
    29u8 => (0, 135, 95),
    30u8 => (0, 135, 135),
    31u8 => (0, 135, 175),
    32u8 => (0, 135, 215),
    33u8 => (0, 135, 255),
    34u8 => (0, 175, 0),
    35u8 => (0, 175, 95),
    36u8 => (0, 175, 135),
    37u8 => (0, 175, 175),
    38u8 => (0, 175, 215),
    39u8 => (0, 175, 255),
    40u8 => (0, 215, 0),
    41u8 => (0, 215, 95),
    42u8 => (0, 215, 135),
    43u8 => (0, 215, 175),
    44u8 => (0, 215, 215),
    45u8 => (0, 215, 255),
    46u8 => (0, 255, 0),
    47u8 => (0, 255, 95),
    48u8 => (0, 255, 135),
    49u8 => (0, 255, 175),
    50u8 => (0, 255, 215),
    51u8 => (0, 255, 255),
    52u8 => (95, 0, 0),
    53u8 => (95, 0, 95),
    54u8 => (95, 0, 135),
    55u8 => (95, 0, 175),
    56u8 => (95, 0, 215),
    57u8 => (95, 0, 255),
    58u8 => (95, 95, 0),
    59u8 => (95, 95, 95),
    60u8 => (95, 95, 135),
    61u8 => (95, 95, 175),
    62u8 => (95, 95, 215),
    63u8 => (95, 95, 255),
    64u8 => (95, 135, 0),
    65u8 => (95, 135, 95),
    66u8 => (95, 135, 135),
    67u8 => (95, 135, 175),
    68u8 => (95, 135, 215),
    69u8 => (95, 135, 255),
    70u8 => (95, 175, 0),
    71u8 => (95, 175, 95),
    72u8 => (95, 175, 135),
    73u8 => (95, 175, 175),
    74u8 => (95, 175, 215),
    75u8 => (95, 175, 255),
    76u8 => (95, 215, 0),
    77u8 => (95, 215, 95),
    78u8 => (95, 215, 135),
    79u8 => (95, 215, 175),
    80u8 => (95, 215, 215),
    81u8 => (95, 215, 255),
    82u8 => (95, 255, 0),
    83u8 => (95, 255, 95),
    84u8 => (95, 255, 135),
    85u8 => (95, 255, 175),
    86u8 => (95, 255, 215),
    87u8 => (95, 255, 255),
    88u8 => (135, 0, 0),
    89u8 => (135, 0, 95),
    90u8 => (135, 0, 135),
    91u8 => (135, 0, 175),
    92u8 => (135, 0, 215),
    93u8 => (135, 0, 255),
    94u8 => (135, 95, 0),
    95u8 => (135, 95, 95),
    96u8 => (135, 95, 135),
    97u8 => (135, 95, 175),
    98u8 => (135, 95, 215),
    99u8 => (135, 95, 255),
    100u8 => (135, 135, 0),
    101u8 => (135, 135, 95),
    102u8 => (135, 135, 135),
    103u8 => (135, 135, 175),
    104u8 => (135, 135, 215),
    105u8 => (135, 135, 255),
    106u8 => (135, 175, 0),
    107u8 => (135, 175, 95),
    108u8 => (135, 175, 135),
    109u8 => (135, 175, 175),
    110u8 => (135, 175, 215),
    111u8 => (135, 175, 255),
    112u8 => (135, 215, 0),
    113u8 => (135, 215, 95),
    114u8 => (135, 215, 135),
    115u8 => (135, 215, 175),
    116u8 => (135, 215, 215),
    117u8 => (135, 215, 255),
    118u8 => (135, 255, 0),
    119u8 => (135, 255, 95),
    120u8 => (135, 255, 135),
    121u8 => (135, 255, 175),
    122u8 => (135, 255, 215),
    123u8 => (135, 255, 255),
    124u8 => (175, 0, 0),
    125u8 => (175, 0, 95),
    126u8 => (175, 0, 135),
    127u8 => (175, 0, 175),
    128u8 => (175, 0, 215),
    129u8 => (175, 0, 255),
    130u8 => (175, 95, 0),
    131u8 => (175, 95, 95),
    132u8 => (175, 95, 135),
    133u8 => (175, 95, 175),
    134u8 => (175, 95, 215),
    135u8 => (175, 95, 255),
    136u8 => (175, 135, 0),
    137u8 => (175, 135, 95),
    138u8 => (175, 135, 135),
    139u8 => (175, 135, 175),
    140u8 => (175, 135, 215),
    141u8 => (175, 135, 255),
    142u8 => (175, 175, 0),
    143u8 => (175, 175, 95),
    144u8 => (175, 175, 135),
    145u8 => (175, 175, 175),
    146u8 => (175, 175, 215),
    147u8 => (175, 175, 255),
    148u8 => (175, 215, 0),
    149u8 => (175, 215, 95),
    150u8 => (175, 215, 135),
    151u8 => (175, 215, 175),
    152u8 => (175, 215, 215),
    153u8 => (175, 215, 255),
    154u8 => (175, 255, 0),
    155u8 => (175, 255, 95),
    156u8 => (175, 255, 135),
    157u8 => (175, 255, 175),
    158u8 => (175, 255, 215),
    159u8 => (175, 255, 255),
    160u8 => (215, 0, 0),
    161u8 => (215, 0, 95),
    162u8 => (215, 0, 135),
    163u8 => (215, 0, 175),
    164u8 => (215, 0, 215),
    165u8 => (215, 0, 255),
    166u8 => (215, 95, 0),
    167u8 => (215, 95, 95),
    168u8 => (215, 95, 135),
    169u8 => (215, 95, 175),
    170u8 => (215, 95, 215),
    171u8 => (215, 95, 255),
    172u8 => (215, 135, 0),
    173u8 => (215, 135, 95),
    174u8 => (215, 135, 135),
    175u8 => (215, 135, 175),
    176u8 => (215, 135, 215),
    177u8 => (215, 135, 255),
    178u8 => (215, 175, 0),
    179u8 => (215, 175, 95),
    180u8 => (215, 175, 135),
    181u8 => (215, 175, 175),
    182u8 => (215, 175, 215),
    183u8 => (215, 175, 255),
    184u8 => (215, 215, 0),
    185u8 => (215, 215, 95),
    186u8 => (215, 215, 135),
    187u8 => (215, 215, 175),
    188u8 => (215, 215, 215),
    189u8 => (215, 215, 255),
    190u8 => (215, 255, 0),
    191u8 => (215, 255, 95),
    192u8 => (215, 255, 135),
    193u8 => (215, 255, 175),
    194u8 => (215, 255, 215),
    195u8 => (215, 255, 255),
    196u8 => (255, 0, 0),
    197u8 => (255, 0, 95),
    198u8 => (255, 0, 135),
    199u8 => (255, 0, 175),
    200u8 => (255, 0, 215),
    201u8 => (255, 0, 255),
    202u8 => (255, 95, 0),
    203u8 => (255, 95, 95),
    204u8 => (255, 95, 135),
    205u8 => (255, 95, 175),
    206u8 => (255, 95, 215),
    207u8 => (255, 95, 255),
    208u8 => (255, 135, 0),
    209u8 => (255, 135, 95),
    210u8 => (255, 135, 135),
    211u8 => (255, 135, 175),
    212u8 => (255, 135, 215),
    213u8 => (255, 135, 255),
    214u8 => (255, 175, 0),
    215u8 => (255, 175, 95),
    216u8 => (255, 175, 135),
    217u8 => (255, 175, 175),
    218u8 => (255, 175, 215),
    219u8 => (255, 175, 255),
    220u8 => (255, 215, 0),
    221u8 => (255, 215, 95),
    222u8 => (255, 215, 135),
    223u8 => (255, 215, 175),
    224u8 => (255, 215, 215),
    225u8 => (255, 215, 255),
    226u8 => (255, 255, 0),
    227u8 => (255, 255, 95),
    228u8 => (255, 255, 135),
    229u8 => (255, 255, 175),
    230u8 => (255, 255, 215),
    231u8 => (255, 255, 255),
    232u8 => (8, 8, 8),
    233u8 => (18, 18, 18),
    234u8 => (28, 28, 28),
    235u8 => (38, 38, 38),
    236u8 => (48, 48, 48),
    237u8 => (58, 58, 58),
    238u8 => (68, 68, 68),
    239u8 => (78, 78, 78),
    240u8 => (88, 88, 88),
    241u8 => (98, 98, 98),
    242u8 => (108, 108, 108),
    243u8 => (118, 118, 118),
    244u8 => (128, 128, 128),
    245u8 => (138, 138, 138),
    246u8 => (148, 148, 148),
    247u8 => (158, 158, 158),
    248u8 => (168, 168, 168),
    249u8 => (178, 178, 178),
    250u8 => (188, 188, 188),
    251u8 => (198, 198, 198),
    252u8 => (208, 208, 208),
    253u8 => (218, 218, 218),
    254u8 => (228, 228, 228),
    255u8 => (238, 238, 238),
};