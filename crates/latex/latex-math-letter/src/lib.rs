#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LxMathLetter {
    UpperA,
    UpperB,
    UpperC,
    UpperD,
    UpperE,
    UpperF,
    UpperG,
    UpperH,
    UpperI,
    UpperJ,
    UpperK,
    UpperL,
    UpperM,
    UpperN,
    UpperO,
    UpperP,
    UpperQ,
    UpperR,
    UpperS,
    UpperT,
    UpperU,
    UpperV,
    UpperW,
    UpperX,
    UpperY,
    UpperZ,
    LowerA,
    LowerB,
    LowerC,
    LowerD,
    LowerE,
    LowerF,
    LowerG,
    LowerH,
    LowerI,
    LowerJ,
    LowerK,
    LowerL,
    LowerM,
    LowerN,
    LowerO,
    LowerP,
    LowerQ,
    LowerR,
    LowerS,
    LowerT,
    LowerU,
    LowerV,
    LowerW,
    LowerX,
    LowerY,
    LowerZ,
    MathcalA,
    MathcalB,
    MathcalC,
    MathcalD,
    MathcalE,
    MathcalF,
    MathcalG,
    MathcalH,
    MathcalI,
    MathcalJ,
    MathcalK,
    MathcalL,
    MathcalM,
    MathcalN,
    MathcalO,
    MathcalP,
    MathcalQ,
    MathcalR,
    MathcalS,
    MathcalT,
    MathcalU,
    MathcalV,
    MathcalW,
    MathcalX,
    MathcalY,
    MathcalZ,
    MathbbA,
    MathbbB,
    MathbbC,
    MathbbD,
    MathbbE,
    MathbbF,
    MathbbG,
    MathbbH,
    MathbbI,
    MathbbJ,
    MathbbK,
    MathbbL,
    MathbbM,
    MathbbN,
    MathbbO,
    MathbbP,
    MathbbQ,
    MathbbR,
    MathbbS,
    MathbbT,
    MathbbU,
    MathbbV,
    MathbbW,
    MathbbX,
    MathbbY,
    MathbbZ,
    MathfrakA,
    MathfrakB,
    MathfrakC,
    MathfrakD,
    MathfrakE,
    MathfrakF,
    MathfrakG,
    MathfrakH,
    MathfrakI,
    MathfrakJ,
    MathfrakK,
    MathfrakL,
    MathfrakM,
    MathfrakN,
    MathfrakO,
    MathfrakP,
    MathfrakQ,
    MathfrakR,
    MathfrakS,
    MathfrakT,
    MathfrakU,
    MathfrakV,
    MathfrakW,
    MathfrakX,
    MathfrakY,
    MathfrakZ,
    UpperGamma,
    UpperDelta,
    UpperTheta,
    UpperLambda,
    UpperXi,
    UpperPi,
    UpperSigma,
    UpperPhi,
    UpperPsi,
    UpperOmega,
    LowerAlpha,
    LowerBeta,
    LowerGamma,
    LowerDelta,
    LowerEpsilon,
    LowerZeta,
    LowerEta,
    LowerTheta,
    LowerIota,
    LowerKappa,
    LowerLambda,
    LowerMu,
    LowerNu,
    LowerXi,
    LowerOmicron,
    LowerPi,
    LowerRho,
    LowerSigma,
    LowerTau,
    LowerUpsilon,
    LowerPhi,
    LowerChi,
    LowerPsi,
    LowerOmega,
}

impl LxMathLetter {
    pub fn latex_code(self) -> &'static str {
        match self {
            LxMathLetter::UpperA => "A",
            LxMathLetter::UpperB => "B",
            LxMathLetter::UpperC => "C",
            LxMathLetter::UpperD => "D",
            LxMathLetter::UpperE => "E",
            LxMathLetter::UpperF => "F",
            LxMathLetter::UpperG => "G",
            LxMathLetter::UpperH => "H",
            LxMathLetter::UpperI => "I",
            LxMathLetter::UpperJ => "J",
            LxMathLetter::UpperK => "K",
            LxMathLetter::UpperL => "L",
            LxMathLetter::UpperM => "M",
            LxMathLetter::UpperN => "N",
            LxMathLetter::UpperO => "O",
            LxMathLetter::UpperP => "P",
            LxMathLetter::UpperQ => "Q",
            LxMathLetter::UpperR => "R",
            LxMathLetter::UpperS => "S",
            LxMathLetter::UpperT => "T",
            LxMathLetter::UpperU => "U",
            LxMathLetter::UpperV => "V",
            LxMathLetter::UpperW => "W",
            LxMathLetter::UpperX => "X",
            LxMathLetter::UpperY => "Y",
            LxMathLetter::UpperZ => "Z",
            LxMathLetter::LowerA => "a",
            LxMathLetter::LowerB => "b",
            LxMathLetter::LowerC => "c",
            LxMathLetter::LowerD => "d",
            LxMathLetter::LowerE => "e",
            LxMathLetter::LowerF => "f",
            LxMathLetter::LowerG => "g",
            LxMathLetter::LowerH => "h",
            LxMathLetter::LowerI => "i",
            LxMathLetter::LowerJ => "j",
            LxMathLetter::LowerK => "k",
            LxMathLetter::LowerL => "l",
            LxMathLetter::LowerM => "m",
            LxMathLetter::LowerN => "n",
            LxMathLetter::LowerO => "o",
            LxMathLetter::LowerP => "p",
            LxMathLetter::LowerQ => "q",
            LxMathLetter::LowerR => "r",
            LxMathLetter::LowerS => "s",
            LxMathLetter::LowerT => "t",
            LxMathLetter::LowerU => "u",
            LxMathLetter::LowerV => "v",
            LxMathLetter::LowerW => "w",
            LxMathLetter::LowerX => "x",
            LxMathLetter::LowerY => "y",
            LxMathLetter::LowerZ => "z",
            LxMathLetter::MathcalA => "\\mathcal{A}",
            LxMathLetter::MathcalB => "\\mathcal{B}",
            LxMathLetter::MathcalC => "\\mathcal{C}",
            LxMathLetter::MathcalD => "\\mathcal{D}",
            LxMathLetter::MathcalE => "\\mathcal{E}",
            LxMathLetter::MathcalF => "\\mathcal{F}",
            LxMathLetter::MathcalG => "\\mathcal{G}",
            LxMathLetter::MathcalH => "\\mathcal{H}",
            LxMathLetter::MathcalI => "\\mathcal{I}",
            LxMathLetter::MathcalJ => "\\mathcal{J}",
            LxMathLetter::MathcalK => "\\mathcal{K}",
            LxMathLetter::MathcalL => "\\mathcal{L}",
            LxMathLetter::MathcalM => "\\mathcal{M}",
            LxMathLetter::MathcalN => "\\mathcal{N}",
            LxMathLetter::MathcalO => "\\mathcal{O}",
            LxMathLetter::MathcalP => "\\mathcal{P}",
            LxMathLetter::MathcalQ => "\\mathcal{Q}",
            LxMathLetter::MathcalR => "\\mathcal{R}",
            LxMathLetter::MathcalS => "\\mathcal{S}",
            LxMathLetter::MathcalT => "\\mathcal{T}",
            LxMathLetter::MathcalU => "\\mathcal{U}",
            LxMathLetter::MathcalV => "\\mathcal{V}",
            LxMathLetter::MathcalW => "\\mathcal{W}",
            LxMathLetter::MathcalX => "\\mathcal{X}",
            LxMathLetter::MathcalY => "\\mathcal{Y}",
            LxMathLetter::MathcalZ => "\\mathcal{Z}",
            LxMathLetter::MathbbA => "\\mathbb{A}",
            LxMathLetter::MathbbB => "\\mathbb{B}",
            LxMathLetter::MathbbC => "\\mathbb{C}",
            LxMathLetter::MathbbD => "\\mathbb{D}",
            LxMathLetter::MathbbE => "\\mathbb{E}",
            LxMathLetter::MathbbF => "\\mathbb{F}",
            LxMathLetter::MathbbG => "\\mathbb{G}",
            LxMathLetter::MathbbH => "\\mathbb{H}",
            LxMathLetter::MathbbI => "\\mathbb{I}",
            LxMathLetter::MathbbJ => "\\mathbb{J}",
            LxMathLetter::MathbbK => "\\mathbb{K}",
            LxMathLetter::MathbbL => "\\mathbb{L}",
            LxMathLetter::MathbbM => "\\mathbb{M}",
            LxMathLetter::MathbbN => "\\mathbb{N}",
            LxMathLetter::MathbbO => "\\mathbb{O}",
            LxMathLetter::MathbbP => "\\mathbb{P}",
            LxMathLetter::MathbbQ => "\\mathbb{Q}",
            LxMathLetter::MathbbR => "\\mathbb{R}",
            LxMathLetter::MathbbS => "\\mathbb{S}",
            LxMathLetter::MathbbT => "\\mathbb{T}",
            LxMathLetter::MathbbU => "\\mathbb{U}",
            LxMathLetter::MathbbV => "\\mathbb{V}",
            LxMathLetter::MathbbW => "\\mathbb{W}",
            LxMathLetter::MathbbX => "\\mathbb{X}",
            LxMathLetter::MathbbY => "\\mathbb{Y}",
            LxMathLetter::MathbbZ => "\\mathbb{Z}",
            LxMathLetter::MathfrakA => "\\mathfrak{A}",
            LxMathLetter::MathfrakB => "\\mathfrak{B}",
            LxMathLetter::MathfrakC => "\\mathfrak{C}",
            LxMathLetter::MathfrakD => "\\mathfrak{D}",
            LxMathLetter::MathfrakE => "\\mathfrak{E}",
            LxMathLetter::MathfrakF => "\\mathfrak{F}",
            LxMathLetter::MathfrakG => "\\mathfrak{G}",
            LxMathLetter::MathfrakH => "\\mathfrak{H}",
            LxMathLetter::MathfrakI => "\\mathfrak{I}",
            LxMathLetter::MathfrakJ => "\\mathfrak{J}",
            LxMathLetter::MathfrakK => "\\mathfrak{K}",
            LxMathLetter::MathfrakL => "\\mathfrak{L}",
            LxMathLetter::MathfrakM => "\\mathfrak{M}",
            LxMathLetter::MathfrakN => "\\mathfrak{N}",
            LxMathLetter::MathfrakO => "\\mathfrak{O}",
            LxMathLetter::MathfrakP => "\\mathfrak{P}",
            LxMathLetter::MathfrakQ => "\\mathfrak{Q}",
            LxMathLetter::MathfrakR => "\\mathfrak{R}",
            LxMathLetter::MathfrakS => "\\mathfrak{S}",
            LxMathLetter::MathfrakT => "\\mathfrak{T}",
            LxMathLetter::MathfrakU => "\\mathfrak{U}",
            LxMathLetter::MathfrakV => "\\mathfrak{V}",
            LxMathLetter::MathfrakW => "\\mathfrak{W}",
            LxMathLetter::MathfrakX => "\\mathfrak{X}",
            LxMathLetter::MathfrakY => "\\mathfrak{Y}",
            LxMathLetter::MathfrakZ => "\\mathfrak{Z}",
            LxMathLetter::UpperGamma => "\\Gamma",
            LxMathLetter::UpperDelta => "\\Delta",
            LxMathLetter::UpperTheta => "\\Theta",
            LxMathLetter::UpperLambda => "\\Lambda",
            LxMathLetter::UpperXi => "\\Xi",
            LxMathLetter::UpperPi => "\\Pi",
            LxMathLetter::UpperSigma => "\\Sigma",
            LxMathLetter::UpperPhi => "\\Phi",
            LxMathLetter::UpperPsi => "\\Psi",
            LxMathLetter::UpperOmega => "\\Omega",
            LxMathLetter::LowerAlpha => "\\alpha",
            LxMathLetter::LowerBeta => "\\beta",
            LxMathLetter::LowerGamma => "\\gamma",
            LxMathLetter::LowerDelta => "\\delta",
            LxMathLetter::LowerEpsilon => "\\epsilon",
            LxMathLetter::LowerZeta => "\\zeta",
            LxMathLetter::LowerEta => "\\eta",
            LxMathLetter::LowerTheta => "\\theta",
            LxMathLetter::LowerIota => "\\iota",
            LxMathLetter::LowerKappa => "\\kappa",
            LxMathLetter::LowerLambda => "\\lambda",
            LxMathLetter::LowerMu => "\\mu",
            LxMathLetter::LowerNu => "\\nu",
            LxMathLetter::LowerXi => "\\xi",
            LxMathLetter::LowerOmicron => "o", // No special LaTeX command for omicron
            LxMathLetter::LowerPi => "\\pi",
            LxMathLetter::LowerRho => "\\rho",
            LxMathLetter::LowerSigma => "\\sigma",
            LxMathLetter::LowerTau => "\\tau",
            LxMathLetter::LowerUpsilon => "\\upsilon",
            LxMathLetter::LowerPhi => "\\phi",
            LxMathLetter::LowerChi => "\\chi",
            LxMathLetter::LowerPsi => "\\psi",
            LxMathLetter::LowerOmega => "\\omega",
        }
    }

    pub fn try_from_char(c: char) -> Option<Self> {
        Some(match c {
            'A' => LxMathLetter::UpperA,
            'B' => LxMathLetter::UpperB,
            'C' => LxMathLetter::UpperC,
            'D' => LxMathLetter::UpperD,
            'E' => LxMathLetter::UpperE,
            'F' => LxMathLetter::UpperF,
            'G' => LxMathLetter::UpperG,
            'H' => LxMathLetter::UpperH,
            'I' => LxMathLetter::UpperI,
            'J' => LxMathLetter::UpperJ,
            'K' => LxMathLetter::UpperK,
            'L' => LxMathLetter::UpperL,
            'M' => LxMathLetter::UpperM,
            'N' => LxMathLetter::UpperN,
            'O' => LxMathLetter::UpperO,
            'P' => LxMathLetter::UpperP,
            'Q' => LxMathLetter::UpperQ,
            'R' => LxMathLetter::UpperR,
            'S' => LxMathLetter::UpperS,
            'T' => LxMathLetter::UpperT,
            'U' => LxMathLetter::UpperU,
            'V' => LxMathLetter::UpperV,
            'W' => LxMathLetter::UpperW,
            'X' => LxMathLetter::UpperX,
            'Y' => LxMathLetter::UpperY,
            'Z' => LxMathLetter::UpperZ,
            'a' => LxMathLetter::LowerA,
            'b' => LxMathLetter::LowerB,
            'c' => LxMathLetter::LowerC,
            'd' => LxMathLetter::LowerD,
            'e' => LxMathLetter::LowerE,
            'f' => LxMathLetter::LowerF,
            'g' => LxMathLetter::LowerG,
            'h' => LxMathLetter::LowerH,
            'i' => LxMathLetter::LowerI,
            'j' => LxMathLetter::LowerJ,
            'k' => LxMathLetter::LowerK,
            'l' => LxMathLetter::LowerL,
            'm' => LxMathLetter::LowerM,
            'n' => LxMathLetter::LowerN,
            'o' => LxMathLetter::LowerO,
            'p' => LxMathLetter::LowerP,
            'q' => LxMathLetter::LowerQ,
            'r' => LxMathLetter::LowerR,
            's' => LxMathLetter::LowerS,
            't' => LxMathLetter::LowerT,
            'u' => LxMathLetter::LowerU,
            'v' => LxMathLetter::LowerV,
            'w' => LxMathLetter::LowerW,
            'x' => LxMathLetter::LowerX,
            'y' => LxMathLetter::LowerY,
            'z' => LxMathLetter::LowerZ,
            'Γ' => LxMathLetter::UpperGamma,
            'Δ' => LxMathLetter::UpperDelta,
            'Θ' => LxMathLetter::UpperTheta,
            'Λ' => LxMathLetter::UpperLambda,
            'Ξ' => LxMathLetter::UpperXi,
            'Π' => LxMathLetter::UpperPi,
            'Σ' => LxMathLetter::UpperSigma,
            'Φ' => LxMathLetter::UpperPhi,
            'Ψ' => LxMathLetter::UpperPsi,
            'Ω' => LxMathLetter::UpperOmega,
            'α' => LxMathLetter::LowerAlpha,
            'β' => LxMathLetter::LowerBeta,
            'γ' => LxMathLetter::LowerGamma,
            'δ' => LxMathLetter::LowerDelta,
            'ε' => LxMathLetter::LowerEpsilon,
            'ζ' => LxMathLetter::LowerZeta,
            'η' => LxMathLetter::LowerEta,
            'θ' => LxMathLetter::LowerTheta,
            'ι' => LxMathLetter::LowerIota,
            'κ' => LxMathLetter::LowerKappa,
            'λ' => LxMathLetter::LowerLambda,
            'μ' => LxMathLetter::LowerMu,
            'ν' => LxMathLetter::LowerNu,
            'ξ' => LxMathLetter::LowerXi,
            'ο' => LxMathLetter::LowerOmicron,
            'π' => LxMathLetter::LowerPi,
            'ρ' => LxMathLetter::LowerRho,
            'σ' => LxMathLetter::LowerSigma,
            'τ' => LxMathLetter::LowerTau,
            'υ' => LxMathLetter::LowerUpsilon,
            'φ' => LxMathLetter::LowerPhi,
            'χ' => LxMathLetter::LowerChi,
            'ψ' => LxMathLetter::LowerPsi,
            'ω' => LxMathLetter::LowerOmega,
            _ => return None,
        })
    }
}

impl LxMathLetter {
    pub fn to_char(self) -> char {
        match self {
            LxMathLetter::UpperA => 'A',
            LxMathLetter::UpperB => 'B',
            LxMathLetter::UpperC => 'C',
            LxMathLetter::UpperD => 'D',
            LxMathLetter::UpperE => 'E',
            LxMathLetter::UpperF => 'F',
            LxMathLetter::UpperG => 'G',
            LxMathLetter::UpperH => 'H',
            LxMathLetter::UpperI => 'I',
            LxMathLetter::UpperJ => 'J',
            LxMathLetter::UpperK => 'K',
            LxMathLetter::UpperL => 'L',
            LxMathLetter::UpperM => 'M',
            LxMathLetter::UpperN => 'N',
            LxMathLetter::UpperO => 'O',
            LxMathLetter::UpperP => 'P',
            LxMathLetter::UpperQ => 'Q',
            LxMathLetter::UpperR => 'R',
            LxMathLetter::UpperS => 'S',
            LxMathLetter::UpperT => 'T',
            LxMathLetter::UpperU => 'U',
            LxMathLetter::UpperV => 'V',
            LxMathLetter::UpperW => 'W',
            LxMathLetter::UpperX => 'X',
            LxMathLetter::UpperY => 'Y',
            LxMathLetter::UpperZ => 'Z',
            LxMathLetter::LowerA => 'a',
            LxMathLetter::LowerB => 'b',
            LxMathLetter::LowerC => 'c',
            LxMathLetter::LowerD => 'd',
            LxMathLetter::LowerE => 'e',
            LxMathLetter::LowerF => 'f',
            LxMathLetter::LowerG => 'g',
            LxMathLetter::LowerH => 'h',
            LxMathLetter::LowerI => 'i',
            LxMathLetter::LowerJ => 'j',
            LxMathLetter::LowerK => 'k',
            LxMathLetter::LowerL => 'l',
            LxMathLetter::LowerM => 'm',
            LxMathLetter::LowerN => 'n',
            LxMathLetter::LowerO => 'o',
            LxMathLetter::LowerP => 'p',
            LxMathLetter::LowerQ => 'q',
            LxMathLetter::LowerR => 'r',
            LxMathLetter::LowerS => 's',
            LxMathLetter::LowerT => 't',
            LxMathLetter::LowerU => 'u',
            LxMathLetter::LowerV => 'v',
            LxMathLetter::LowerW => 'w',
            LxMathLetter::LowerX => 'x',
            LxMathLetter::LowerY => 'y',
            LxMathLetter::LowerZ => 'z',
            LxMathLetter::MathcalA => 'A',
            LxMathLetter::MathcalB => 'B',
            LxMathLetter::MathcalC => 'C',
            LxMathLetter::MathcalD => 'D',
            LxMathLetter::MathcalE => 'E',
            LxMathLetter::MathcalF => 'F',
            LxMathLetter::MathcalG => 'G',
            LxMathLetter::MathcalH => 'H',
            LxMathLetter::MathcalI => 'I',
            LxMathLetter::MathcalJ => 'J',
            LxMathLetter::MathcalK => 'K',
            LxMathLetter::MathcalL => 'L',
            LxMathLetter::MathcalM => 'M',
            LxMathLetter::MathcalN => 'N',
            LxMathLetter::MathcalO => 'O',
            LxMathLetter::MathcalP => 'P',
            LxMathLetter::MathcalQ => 'Q',
            LxMathLetter::MathcalR => 'R',
            LxMathLetter::MathcalS => 'S',
            LxMathLetter::MathcalT => 'T',
            LxMathLetter::MathcalU => 'U',
            LxMathLetter::MathcalV => 'V',
            LxMathLetter::MathcalW => 'W',
            LxMathLetter::MathcalX => 'X',
            LxMathLetter::MathcalY => 'Y',
            LxMathLetter::MathcalZ => 'Z',
            LxMathLetter::MathbbA => 'A',
            LxMathLetter::MathbbB => 'B',
            LxMathLetter::MathbbC => 'C',
            LxMathLetter::MathbbD => 'D',
            LxMathLetter::MathbbE => 'E',
            LxMathLetter::MathbbF => 'F',
            LxMathLetter::MathbbG => 'G',
            LxMathLetter::MathbbH => 'H',
            LxMathLetter::MathbbI => 'I',
            LxMathLetter::MathbbJ => 'J',
            LxMathLetter::MathbbK => 'K',
            LxMathLetter::MathbbL => 'L',
            LxMathLetter::MathbbM => 'M',
            LxMathLetter::MathbbN => 'N',
            LxMathLetter::MathbbO => 'O',
            LxMathLetter::MathbbP => 'P',
            LxMathLetter::MathbbQ => 'Q',
            LxMathLetter::MathbbR => 'R',
            LxMathLetter::MathbbS => 'S',
            LxMathLetter::MathbbT => 'T',
            LxMathLetter::MathbbU => 'U',
            LxMathLetter::MathbbV => 'V',
            LxMathLetter::MathbbW => 'W',
            LxMathLetter::MathbbX => 'X',
            LxMathLetter::MathbbY => 'Y',
            LxMathLetter::MathbbZ => 'Z',
            LxMathLetter::MathfrakA => 'a',
            LxMathLetter::MathfrakB => 'b',
            LxMathLetter::MathfrakC => 'c',
            LxMathLetter::MathfrakD => 'd',
            LxMathLetter::MathfrakE => 'e',
            LxMathLetter::MathfrakF => 'f',
            LxMathLetter::MathfrakG => 'g',
            LxMathLetter::MathfrakH => 'h',
            LxMathLetter::MathfrakI => 'i',
            LxMathLetter::MathfrakJ => 'j',
            LxMathLetter::MathfrakK => 'k',
            LxMathLetter::MathfrakL => 'l',
            LxMathLetter::MathfrakM => 'm',
            LxMathLetter::MathfrakN => 'n',
            LxMathLetter::MathfrakO => 'o',
            LxMathLetter::MathfrakP => 'p',
            LxMathLetter::MathfrakQ => 'q',
            LxMathLetter::MathfrakR => 'r',
            LxMathLetter::MathfrakS => 's',
            LxMathLetter::MathfrakT => 't',
            LxMathLetter::MathfrakU => 'u',
            LxMathLetter::MathfrakV => 'v',
            LxMathLetter::MathfrakW => 'w',
            LxMathLetter::MathfrakX => 'x',
            LxMathLetter::MathfrakY => 'y',
            LxMathLetter::MathfrakZ => 'z',
            LxMathLetter::UpperGamma => 'Γ',
            LxMathLetter::UpperDelta => 'Δ',
            LxMathLetter::UpperTheta => 'Θ',
            LxMathLetter::UpperLambda => 'Λ',
            LxMathLetter::UpperXi => 'Ξ',
            LxMathLetter::UpperPi => 'Π',
            LxMathLetter::UpperSigma => 'Σ',
            LxMathLetter::UpperPhi => 'Φ',
            LxMathLetter::UpperPsi => 'Ψ',
            LxMathLetter::UpperOmega => 'Ω',
            LxMathLetter::LowerAlpha => 'α',
            LxMathLetter::LowerBeta => 'β',
            LxMathLetter::LowerGamma => 'γ',
            LxMathLetter::LowerDelta => 'δ',
            LxMathLetter::LowerEpsilon => 'ε',
            LxMathLetter::LowerZeta => 'ζ',
            LxMathLetter::LowerEta => 'η',
            LxMathLetter::LowerTheta => 'θ',
            LxMathLetter::LowerIota => 'ι',
            LxMathLetter::LowerKappa => 'κ',
            LxMathLetter::LowerLambda => 'λ',
            LxMathLetter::LowerMu => 'μ',
            LxMathLetter::LowerNu => 'ν',
            LxMathLetter::LowerXi => 'ξ',
            LxMathLetter::LowerOmicron => 'ο',
            LxMathLetter::LowerPi => 'π',
            LxMathLetter::LowerRho => 'ρ',
            LxMathLetter::LowerSigma => 'σ',
            LxMathLetter::LowerTau => 'τ',
            LxMathLetter::LowerUpsilon => 'υ',
            LxMathLetter::LowerPhi => 'φ',
            LxMathLetter::LowerChi => 'χ',
            LxMathLetter::LowerPsi => 'ψ',
            LxMathLetter::LowerOmega => 'ω',
        }
    }
}