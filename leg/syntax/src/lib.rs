pub mod syntax_kind;
use syntax_kind::SyntaxKind;

impl From<u16> for SyntaxKind {
    #[inline]
    fn from(d: u16) -> SyntaxKind {
        assert!(d <= (SyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(k: SyntaxKind) -> Self {
        k as u16
    }
}

pub type SyntaxNode = rowan::SyntaxNode<WLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<WLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<WLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<WLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<WLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum WLanguage {}

impl rowan::Language for WLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}
