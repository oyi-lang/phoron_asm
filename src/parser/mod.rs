use crate::{
    ast::*,
    diagnostics::DiagnosticManager,
    lexer::{Lexer, Token, TokenKind},
    sourcefile::Span,
};

mod type_descriptor_parser;
use type_descriptor_parser as tdp;

use TokenKind::*;

/// The Phoron parser
pub struct Parser<'p> {
    lexer: Lexer<'p>,
    curr_tok: Token,
    pub errored: bool,
}

impl<'p> Parser<'p> {
    pub fn new(lexer: Lexer<'p>) -> Self {
        Parser {
            lexer,
            curr_tok: Token {
                kind: TokenKind::TEof,
                span: Span::default(),
            },
            errored: false,
        }
    }

    fn curr_span(&self) -> Span {
        self.curr_tok.span
    }

    fn advance(&mut self) -> bool {
        match self.lexer.lex() {
            None => false,
            Some(tok) => {
                self.curr_tok = tok;
                true
            }
        }
    }

    fn advance_if(&mut self, expected_token: &TokenKind) -> bool {
        if self.see().kind != *expected_token {
            false
        } else {
            self.advance()
        }
    }

    fn see(&self) -> &Token {
        &self.curr_tok
    }

    fn is_class_or_interface_access_flag(&self, tok: &TokenKind) -> bool {
        match tok {
            TPublic | TFinal | TSuper | TInterface | TAbstract | TSynthetic | TAnnotation
            | TEnum | TModule => true,
            _ => false,
        }
    }

    fn is_field_access_flag(&self, tok: &TokenKind) -> bool {
        match tok {
            TPublic | TPrivate | TProtected | TStatic | TFinal | TVolatile | TTransient
            | TSynthetic | TEnum => true,
            _ => false,
        }
    }

    fn is_method_access_flag(&self, tok: &TokenKind) -> bool {
        match tok {
            TPublic | TPrivate | TProtected | TStatic | TFinal | TSynthetic | TSynchronized
            | TBridge | TVarargs | TNative | TAbstract | TStrict => true,
            _ => false,
        }
    }

    fn get_class_or_interface_access_flag(
        &self,
        tok: &TokenKind,
    ) -> PhoronClassOrInterfaceAccessFlag {
        match tok {
            TPublic => PhoronClassOrInterfaceAccessFlag::AccPublic,
            TFinal => PhoronClassOrInterfaceAccessFlag::AccFinal,
            TSuper => PhoronClassOrInterfaceAccessFlag::AccSuper,
            TInterface => PhoronClassOrInterfaceAccessFlag::AccInterface,
            TAbstract => PhoronClassOrInterfaceAccessFlag::AccAbstract,
            TSynthetic => PhoronClassOrInterfaceAccessFlag::AccSynthetic,
            TAnnotation => PhoronClassOrInterfaceAccessFlag::AccAnnotation,
            TEnum => PhoronClassOrInterfaceAccessFlag::AccEnum,
            TModule => PhoronClassOrInterfaceAccessFlag::AccModule,
            _ => {
                panic!("unknown class or interface flag")
            }
        }
    }

    fn get_field_access_flags(&self, tok: &TokenKind) -> PhoronFieldAccessFlag {
        match tok {
            TPublic => PhoronFieldAccessFlag::AccPublic,
            TPrivate => PhoronFieldAccessFlag::AccPrivate,
            TProtected => PhoronFieldAccessFlag::AccProtected,
            TStatic => PhoronFieldAccessFlag::AccStatic,
            TFinal => PhoronFieldAccessFlag::AccFinal,
            TVolatile => PhoronFieldAccessFlag::AccVolatile,
            TTransient => PhoronFieldAccessFlag::AccTransient,
            TSynthetic => PhoronFieldAccessFlag::AccSynthetic,
            TEnum => PhoronFieldAccessFlag::AccEnum,
            _ => {
                panic!("unknown field access flag")
            }
        }
    }

    fn get_method_acess_flags(&self, tok: &TokenKind) -> PhoronMethodAccessFlag {
        match tok {
            TPublic => PhoronMethodAccessFlag::AccPublic,
            TPrivate => PhoronMethodAccessFlag::AccPrivate,
            TProtected => PhoronMethodAccessFlag::AccProtected,
            TStatic => PhoronMethodAccessFlag::AccStatic,
            TSynchronized => PhoronMethodAccessFlag::AccSynchronized,
            TFinal => PhoronMethodAccessFlag::AccFinal,
            TBridge => PhoronMethodAccessFlag::AccBridge,
            TVarargs => PhoronMethodAccessFlag::AccVarargs,
            TNative => PhoronMethodAccessFlag::AccNative,
            TAbstract => PhoronMethodAccessFlag::AccAbstract,
            TStrict => PhoronMethodAccessFlag::AccStrict,
            TSynthetic => PhoronMethodAccessFlag::AccSynthetic,
            _ => {
                panic!("unknown method access flag")
            }
        }
    }

    /// ClassDef <- CLASS_keyword AccessFlag* ClassName newline
    fn parse_class_def(&mut self) -> Option<PhoronClassDef> {
        self.advance();

        let mut access_flags = vec![self.get_class_or_interface_access_flag(&TSuper)];

        match &self.see().kind {
            TPublic | TFinal | TSuper | TInterface | TAbstract | TSynthetic | TAnnotation
            | TEnum | TModule => {
                while self.is_class_or_interface_access_flag(&self.see().kind) {
                    access_flags.push(self.get_class_or_interface_access_flag(&self.see().kind));
                    self.advance();
                }

                if let Token {
                    kind: TokenKind::TIdent(name),
                    ..
                } = self.see()
                {
                    let name = name.to_string();
                    self.advance();

                    Some(PhoronClassDef { name, access_flags })
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing class name"),
                    );
                    self.errored = true;
                    None
                }
            }

            TokenKind::TIdent(name) => {
                let name = name.to_string();
                self.advance();

                Some(PhoronClassDef { name, access_flags })
            }

            tok => {
                DiagnosticManager::report_diagnostic(
                    &self.lexer.source_file,
                    self.curr_span(),
                    format!("invalid token {tok:?}"),
                );

                self.errored = true;

                Some(PhoronClassDef::default())
            }
        }
    }

    /// InterfaceDef <- INTERFACE_keyword AccessFlag* ClassName newline
    fn parse_interface_def(&mut self) -> Option<PhoronInterfaceDef> {
        self.advance();

        let curr_tok = self.see();
        let mut access_flags = vec![self.get_class_or_interface_access_flag(&TAbstract)];

        match &curr_tok.kind {
            TPublic | TFinal | TSuper | TInterface | TAbstract | TSynthetic | TAnnotation
            | TEnum | TModule => {
                while self.is_class_or_interface_access_flag(&self.see().kind) {
                    access_flags.push(self.get_class_or_interface_access_flag(&self.see().kind));
                    self.advance();
                }

                if let Token {
                    kind: TokenKind::TIdent(ident),
                    ..
                } = self.see()
                {
                    let name = ident.to_string();
                    self.advance();

                    Some(PhoronInterfaceDef { name, access_flags })
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing interface name"),
                    );
                    self.errored |= true;
                    None
                }
            }

            TokenKind::TIdent(ident) => {
                let name = ident.to_string();
                self.advance();

                Some(PhoronInterfaceDef { name, access_flags })
            }

            tok_kind => {
                DiagnosticManager::report_diagnostic(
                    &self.lexer.source_file,
                    self.curr_span(),
                    format!("invalid token {tok_kind:?}"),
                );
                self.errored |= true;
                Some(PhoronInterfaceDef::default())
            }
        }
    }

    /// ImplementsDef <- IMPLEMENTS_keyword ClassName newline
    fn parse_implements_def(&mut self) -> Option<PhoronImplementsDef> {
        self.advance();

        if let Token {
            kind: TokenKind::TIdent(ident),
            ..
        } = self.see()
        {
            let class_name = ident.to_string();
            self.advance();

            Some(PhoronImplementsDef { class_name })
        } else {
            DiagnosticManager::report_diagnostic(
                &self.lexer.source_file,
                self.curr_span(),
                format!("missing class name"),
            );

            self.errored |= true;

            Some(PhoronImplementsDef::default())
        }
    }

    fn parse_implements_defs(&mut self) -> Option<Vec<PhoronImplementsDef>> {
        let mut impl_defs = Vec::new();

        while let TokenKind::TImplements = self.see().kind {
            impl_defs.push(self.parse_implements_def()?);
        }

        Some(impl_defs)
    }

    /// SuperDef <- SUPER_keyword ClassName newline
    fn parse_super_def(&mut self) -> Option<PhoronSuperDef> {
        self.advance();

        if let TokenKind::TIdent(ref ident) = self.see().kind {
            let super_class_name = ident.to_string();
            self.advance();

            Some(PhoronSuperDef { super_class_name })
        } else {
            DiagnosticManager::report_diagnostic(
                &self.lexer.source_file,
                self.curr_span(),
                format!("missing super class name"),
            );

            self.errored |= true;

            None
        }
    }

    /// FieldIniValue <- Double / Integer / QuotedString
    fn parse_field_init_value(&mut self) -> Option<Option<PhoronFieldInitValue>> {
        if let TokenKind::TAssign = self.see().kind {
            self.advance();

            Some(if let TokenKind::TInt(int) = self.see().kind {
                let ival = int;
                self.advance();
                Some(PhoronFieldInitValue::Integer(ival))
            } else if let TokenKind::TFloat(float) = self.see().kind {
                let fval = float;
                self.advance();
                Some(PhoronFieldInitValue::Double(fval))
            } else if let TokenKind::TString(ref s) = self.see().kind {
                let sval = s.to_owned();
                self.advance();
                Some(PhoronFieldInitValue::QuotedString(sval))
            } else {
                DiagnosticManager::report_diagnostic(
                    &self.lexer.source_file,
                    self.curr_span(),
                    format!("invalid field init value"),
                );

                self.errored |= true;

                Some(PhoronFieldInitValue::default())
            })
        } else {
            Some(None)
        }
    }

    fn parse_field_descriptor(&mut self) -> Option<PhoronFieldDescriptor> {
        if let Token {
            kind: TokenKind::TIdent(ident),
            span,
        } = self.see()
        {
            let mut field_desc_parser = tdp::TypeParser::new(&ident);

            let field_desc = match field_desc_parser.parse_field_descriptor() {
                Err(_err) => {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        span.merge(&self.curr_span()),
                        format!("invalid or malfomed field type descriptor"),
                    );
                    self.errored |= true;

                    PhoronFieldDescriptor::default()
                }

                Ok(desc) => {
                    self.advance();
                    desc
                }
            };

            Some(field_desc)
        } else {
            DiagnosticManager::report_diagnostic(
                &self.lexer.source_file,
                self.curr_span(),
                format!("invalid token for field descriptor"),
            );
            self.errored |= true;

            Some(PhoronFieldDescriptor::default())
        }
    }

    /// FieldDef <- line_comment* FIELD_keyword FieldAccessFlag* FieldName FieldDescriptor (EQ_symbol FieldIniValue)? newline
    fn parse_field_def(&mut self) -> Option<PhoronFieldDef> {
        let start_span = self.curr_span();

        self.advance();

        let mut access_flags = Vec::new();
        while self.is_field_access_flag(&self.see().kind) {
            access_flags.push(self.get_field_access_flags(&self.see().kind));
            self.advance();
        }

        if let Token {
            kind: TokenKind::TIdent(ident),
            ..
        } = self.see()
        {
            let name = ident.to_string();
            self.advance();

            let field_descriptor = self.parse_field_descriptor()?;
            let init_val = self.parse_field_init_value()?;

            Some(PhoronFieldDef {
                name,
                access_flags,
                field_descriptor,
                init_val,
            })
        } else {
            DiagnosticManager::report_diagnostic(
                &self.lexer.source_file,
                start_span.merge(&self.curr_span()),
                format!("malformed field definition"),
            );
            self.errored |= true;

            Some(PhoronFieldDef::default())
        }
    }

    fn parse_field_defs(&mut self) -> Option<Vec<PhoronFieldDef>> {
        let mut field_defs = Vec::new();
        while let TokenKind::TField = self.see().kind {
            field_defs.push(self.parse_field_def()?);
        }

        Some(field_defs)
    }

    fn parse_class_name(&mut self) -> Option<String> {
        if let Token {
            kind: TokenKind::TIdent(classname),
            ..
        } = self.see()
        {
            let classname = classname.to_owned();
            self.advance();

            Some(classname)
        } else {
            DiagnosticManager::report_diagnostic(
                &self.lexer.source_file,
                self.curr_span(),
                format!("malformed class name"),
            );
            self.errored |= true;

            Some(String::default())
        }
    }

    fn parse_label(&mut self) -> Option<String> {
        if let Token {
            kind: TokenKind::TIdent(label),
            ..
        } = self.see()
        {
            let label = label.to_owned();
            self.advance();

            Some(label)
        } else {
            DiagnosticManager::report_diagnostic(
                &self.lexer.source_file,
                self.curr_span(),
                format!("malformed label"),
            );
            self.errored |= true;

            Some(String::default())
        }
    }

    /// Directive <- (LIMIT_keyword (StackDirective / LocalDirective) / ThrowsDirective / LineNumberDirective / VarDirective / CatchDirective) newline
    /// StackDirective <-  STACK_keyword Integer
    /// LocalDirective <- LOCAL_keyword Integer
    /// ThrowsDirective <- THROWS_keyword ClassName
    /// LineNumberDirective <- LINE_keyword Integer
    /// VarDirective <- VAR_keyword Integer IS_keyword VarName FieldDescriptor FROM_keyword Label TO_keyword Label
    /// CatchDirective <- CATCH_keyword ClassName FROM_keyword Label TO_keyword Label USING_keyword Label
    fn parse_directive(&mut self) -> Option<PhoronDirective> {
        Some(match &self.see().kind {
            TokenKind::TLimit => {
                self.advance();

                match self.see().kind {
                    TokenKind::TStack => {
                        self.advance();

                        if let Token {
                            kind: TokenKind::TInt(n),
                            ..
                        } = self.see()
                        {
                            let max_stack = *n as u16;
                            self.advance();
                            PhoronDirective::LimitStack(max_stack)
                        } else {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing numeric value for `.limit stack` directive"),
                            );

                            self.advance();
                            self.errored |= true;

                            PhoronDirective::default()
                        }
                    }

                    TokenKind::TLocals => {
                        self.advance();

                        if let Token {
                            kind: TokenKind::TInt(n),
                            ..
                        } = self.see()
                        {
                            let max_locals = *n as u16;
                            self.advance();
                            PhoronDirective::LimitLocals(max_locals)
                        } else {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing numeric value for `.limit locals` directive"),
                            );

                            self.advance();
                            self.errored |= true;

                            PhoronDirective::default()
                        }
                    }

                    _ => {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            self.curr_span(),
                            format!("invalid directive"),
                        );

                        self.advance();
                        self.errored |= true;

                        PhoronDirective::default()
                    }
                }
            }

            TokenKind::TThrows => {
                self.advance();

                let class_name = self.parse_class_name().or(Some(String::new()))?;
                PhoronDirective::Throws { class_name }
            }

            TokenKind::TLine => {
                self.advance();

                let line_number = self.parse_us().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing line number"),
                    );
                    self.errored |= true;

                    Some(u16::default())
                })?;

                PhoronDirective::LineNumber(line_number)
            }

            TokenKind::TVar => {
                let start_span = self.curr_span();

                self.advance();

                let varnum = self.parse_us().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u16::default())
                })?;

                if !self.advance_if(&TokenKind::TIs) {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `is` keyword"),
                    );
                }

                let name = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing name"),
                    );
                    self.errored |= true;

                    Some(String::new())
                })?;

                let field_descriptor = self.parse_field_descriptor().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing field descriptor"),
                    );

                    self.advance();
                    self.errored |= true;

                    Some(PhoronFieldDescriptor::default())
                })?;

                if !self.advance_if(&TokenKind::TFrom) {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `from` keyword"),
                    );
                    self.errored |= true;
                }

                let from_label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `from` label"),
                    );
                    self.errored |= true;

                    Some(String::new())
                })?;

                if !self.advance_if(&TokenKind::TTo) {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `to` keyword"),
                    );
                    self.errored |= true;
                }

                let to_label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `to` label"),
                    );
                    self.errored |= true;

                    Some(String::new())
                })?;

                PhoronDirective::Var {
                    varnum,
                    name,
                    field_descriptor,
                    from_label,
                    to_label,
                }
            }

            TokenKind::TCatch => {
                let start_span = self.curr_span();

                self.advance();

                let class_name = self.parse_class_name().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing class name"),
                    );
                    self.errored |= true;

                    Some(String::new())
                })?;

                if !self.advance_if(&TokenKind::TFrom) {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `from` keyword"),
                    );
                    self.errored |= true;
                }

                let from_label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `from` label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                if !self.advance_if(&TokenKind::TTo) {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `to` keyword"),
                    );
                    self.errored |= true;
                }

                let to_label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `to` label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                if !self.advance_if(&TokenKind::TUsing) {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `using` keyword"),
                    );
                    self.errored |= true;
                }

                let handler_label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing handler label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                PhoronDirective::Catch {
                    class_name,
                    from_label,
                    to_label,
                    handler_label,
                }
            }

            _ => {
                unreachable!()
            }
        })
    }

    fn parse_ub(&mut self) -> Option<u8> {
        if let TokenKind::TInt(n) = self.see().kind {
            self.advance();
            Some(n as u8)
        } else {
            None
        }
    }

    fn parse_sb(&mut self) -> Option<i8> {
        if let TokenKind::TInt(n) = self.see().kind {
            self.advance();
            Some(n as i8)
        } else {
            None
        }
    }

    fn parse_us(&mut self) -> Option<u16> {
        if let TokenKind::TInt(n) = self.see().kind {
            self.advance();
            Some(n as u16)
        } else {
            None
        }
    }

    fn parse_ss(&mut self) -> Option<i16> {
        if let TokenKind::TInt(n) = self.see().kind {
            self.advance();
            Some(n as i16)
        } else {
            None
        }
    }

    fn parse_si(&mut self) -> Option<i32> {
        if let TokenKind::TInt(n) = self.see().kind {
            self.advance();
            Some(n as i32)
        } else {
            None
        }
    }

    fn parse_table_switches(&mut self) -> Option<Vec<String>> {
        let mut switches = Vec::new();

        while let TokenKind::TIdent(ref label) = self.see().kind {
            let label = label.to_string();
            self.advance();

            switches.push(label);
        }

        Some(switches)
    }

    fn parse_lookup_switches(&mut self) -> Option<Vec<LookupSwitchPair>> {
        let mut switches = Vec::new();

        while let TokenKind::TInt(key) = self.see().kind {
            self.advance();

            if let TokenKind::TColon = self.see().kind {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label for switch entry"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                switches.push(LookupSwitchPair {
                    key: key as i32,
                    label,
                });
            } else {
                DiagnosticManager::report_diagnostic(
                    &self.lexer.source_file,
                    self.curr_span(),
                    format!("missing : in lookupswitch pair"),
                );
                self.errored |= true;
            }
        }

        Some(switches)
    }

    fn parse_default_switch_pair(&mut self) -> Option<String> {
        if let TokenKind::TDefault = self.see().kind {
            self.advance();

            if let TokenKind::TColon = self.see().kind {
                self.advance();

                let label = self.parse_label()?;
                Some(label)
            } else {
                DiagnosticManager::report_diagnostic(
                    &self.lexer.source_file,
                    self.curr_span(),
                    format!("missing : in default switch pair"),
                );
                self.errored |= true;

                None
            }
        } else {
            DiagnosticManager::report_diagnostic(
                &self.lexer.source_file,
                self.curr_span(),
                format!("missing default keyword"),
            );
            self.errored |= true;

            Some(String::default())
        }
    }

    fn parse_jvm_instruction(&mut self) -> Option<JvmInstruction> {
        Some(match self.see().kind {
            // aaload
            TokenKind::TAaload => {
                self.advance();
                JvmInstruction::Aaload
            }

            // aastore
            TokenKind::TAastore => {
                self.advance();
                JvmInstruction::Aastore
            }

            // aconst_null
            TokenKind::TAconstnull => {
                self.advance();
                JvmInstruction::Aconstnull
            }

            // aload <varnum>
            TokenKind::TAload => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Aload { varnum }
            }

            // aload_0
            TokenKind::TAload0 => {
                self.advance();
                JvmInstruction::Aload0
            }

            // aload_1
            TokenKind::TAload1 => {
                self.advance();
                JvmInstruction::Aload1
            }

            // aload_2
            TokenKind::TAload2 => {
                self.advance();
                JvmInstruction::Aload2
            }

            // aload_3
            TokenKind::TAload3 => {
                self.advance();
                JvmInstruction::Aload3
            }

            // anewarray <type>
            // AnewarrayTypeDescriptor <- ClassType / ArrayType
            TokenKind::TAnewarray => {
                self.advance();

                let component_type = self.parse_field_descriptor().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing component type"),
                    );
                    self.errored |= true;

                    Some(PhoronFieldDescriptor::default())
                })?;

                JvmInstruction::Anewarray { component_type }
            }

            // areturn
            TokenKind::TAreturn => {
                self.advance();
                JvmInstruction::Areturn
            }

            // arraylength
            TokenKind::TArraylength => {
                self.advance();
                JvmInstruction::Arraylength
            }

            // astore <varnum>
            TokenKind::TAstore => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Astore { varnum }
            }

            // astore_0
            TokenKind::TAstore0 => {
                self.advance();
                JvmInstruction::Astore0
            }

            // astore_1
            TokenKind::TAstore1 => {
                self.advance();
                JvmInstruction::Astore1
            }

            // astore_2
            TokenKind::TAstore2 => {
                self.advance();
                JvmInstruction::Astore2
            }

            // astore_3
            TokenKind::TAstore3 => {
                self.advance();
                JvmInstruction::Astore3
            }

            // athrow
            TokenKind::TAthrow => {
                self.advance();
                JvmInstruction::Athrow
            }

            // baload
            TokenKind::TBaload => {
                self.advance();
                JvmInstruction::Baload
            }

            // bastore
            TokenKind::TBastore => {
                self.advance();
                JvmInstruction::Bastore
            }

            // bipush i8
            TokenKind::TBipush => {
                self.advance();

                let sb = self.parse_sb().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing bute constant"),
                    );
                    self.errored |= true;

                    Some(i8::default())
                })?;

                JvmInstruction::Bipush(sb)
            }

            // caload
            TokenKind::TCaload => {
                self.advance();
                JvmInstruction::Caload
            }

            // castore
            TokenKind::TCastore => {
                self.advance();
                JvmInstruction::Castore
            }

            // checkcast <type>
            TokenKind::TCheckcast => {
                self.advance();

                let cast_type = self.parse_field_descriptor().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing cast type"),
                    );
                    self.errored |= true;

                    Some(PhoronFieldDescriptor::default())
                })?;

                JvmInstruction::Checkcast { cast_type }
            }

            // d2f
            TokenKind::TD2f => {
                self.advance();
                JvmInstruction::D2f
            }

            // d2i
            TokenKind::TD2i => {
                self.advance();
                JvmInstruction::D2i
            }

            // d2l
            TokenKind::TD2l => {
                self.advance();
                JvmInstruction::D2l
            }

            // dadd
            TokenKind::TDadd => {
                self.advance();
                JvmInstruction::Dadd
            }

            // daload
            TokenKind::TDaload => {
                self.advance();
                JvmInstruction::Daload
            }

            // dastore
            TokenKind::TDastore => {
                self.advance();
                JvmInstruction::Dastore
            }

            // dcmpg
            TokenKind::TDcmpg => {
                self.advance();
                JvmInstruction::Dcmpg
            }

            // dcmpl
            TokenKind::TDcmpl => {
                self.advance();
                JvmInstruction::Dcmpl
            }

            // dconst_0
            TokenKind::TDconst0 => {
                self.advance();
                JvmInstruction::Dconst0
            }

            // dconst_1
            TokenKind::TDconst1 => {
                self.advance();
                JvmInstruction::Dconst1
            }

            // ddiv
            TokenKind::TDdiv => {
                self.advance();
                JvmInstruction::Ddiv
            }

            // dload <arnum>
            TokenKind::TDload => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("misisng var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Dload { varnum }
            }

            // dload_0
            TokenKind::TDload0 => {
                self.advance();
                JvmInstruction::Dload0
            }

            // dload_1
            TokenKind::TDload1 => {
                self.advance();
                JvmInstruction::Dload1
            }

            // dload_2
            TokenKind::TDload2 => {
                self.advance();
                JvmInstruction::Dload2
            }

            // dload_3
            TokenKind::TDload3 => {
                self.advance();
                JvmInstruction::Dload3
            }

            // dmul
            TokenKind::TDmul => {
                self.advance();
                JvmInstruction::Dmul
            }

            // dneg
            TokenKind::TDneg => {
                self.advance();
                JvmInstruction::Dneg
            }

            // drem
            TokenKind::TDrem => {
                self.advance();
                JvmInstruction::Drem
            }

            // dreturn
            TokenKind::TDreturn => {
                self.advance();
                JvmInstruction::Dreturn
            }

            // dstore< <varnum>
            TokenKind::TDstore => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Dstore { varnum }
            }

            // dstore_0
            TokenKind::TDstore0 => {
                self.advance();
                JvmInstruction::Dstore0
            }

            // dstore_1
            TokenKind::TDstore1 => {
                self.advance();
                JvmInstruction::Dstore1
            }

            // dstore_2
            TokenKind::TDstore2 => {
                self.advance();
                JvmInstruction::Dstore2
            }

            // dstore_3
            TokenKind::TDstore3 => {
                self.advance();
                JvmInstruction::Dstore3
            }

            // dsub
            TokenKind::TDsub => {
                self.advance();
                JvmInstruction::Dsub
            }

            // dup
            TokenKind::TDup => {
                self.advance();
                JvmInstruction::Dup
            }

            // dup2
            TokenKind::TDup2 => {
                self.advance();
                JvmInstruction::Dup2
            }

            // dup2_x1
            TokenKind::TDup2x1 => {
                self.advance();
                JvmInstruction::Dup2x1
            }

            // dup2_x2
            TokenKind::TDup2x2 => {
                self.advance();
                JvmInstruction::Dup2x2
            }

            // dup_x1
            TokenKind::TDupx1 => {
                self.advance();
                JvmInstruction::Dupx1
            }

            // dup_x2
            TokenKind::TDupx2 => {
                self.advance();
                JvmInstruction::Dupx2
            }

            // f2d
            TokenKind::TF2d => {
                self.advance();
                JvmInstruction::F2d
            }

            // f2i
            TokenKind::TF2i => {
                self.advance();
                JvmInstruction::F2i
            }

            // f2l
            TokenKind::TF2l => {
                self.advance();
                JvmInstruction::F2l
            }

            // fadd
            TokenKind::TFadd => {
                self.advance();
                JvmInstruction::Fadd
            }

            // faload
            TokenKind::TFaload => {
                self.advance();
                JvmInstruction::Faload
            }

            // fastore
            TokenKind::TFastore => {
                self.advance();
                JvmInstruction::Fastore
            }

            // fcmpg
            TokenKind::TFcmpg => {
                self.advance();
                JvmInstruction::Fcmpg
            }

            // fcmpl
            TokenKind::TFcmpl => {
                self.advance();
                JvmInstruction::Fcmpl
            }

            // fconst_0
            TokenKind::TFconst0 => {
                self.advance();
                JvmInstruction::Fconst0
            }

            // fconst_1
            TokenKind::TFconst1 => {
                self.advance();
                JvmInstruction::Fconst1
            }

            // fconst_2
            TokenKind::TFconst2 => {
                self.advance();
                JvmInstruction::Fconst2
            }

            // fdiv
            TokenKind::TFdiv => {
                self.advance();
                JvmInstruction::Fdiv
            }

            // fload <varnum>
            TokenKind::TFload => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Fload { varnum }
            }

            // fload_0
            TokenKind::TFload0 => {
                self.advance();
                JvmInstruction::Fload0
            }

            // fload_1
            TokenKind::TFload1 => {
                self.advance();
                JvmInstruction::Fload1
            }

            // fload_2
            TokenKind::TFload2 => {
                self.advance();
                JvmInstruction::Fload2
            }

            // fload_3
            TokenKind::TFload3 => {
                self.advance();
                JvmInstruction::Fload3
            }

            // fmul
            TokenKind::TFmul => {
                self.advance();
                JvmInstruction::Fmul
            }

            // fneg
            TokenKind::TFneg => {
                self.advance();
                JvmInstruction::Fneg
            }

            // frem
            TokenKind::TFrem => {
                self.advance();
                JvmInstruction::Frem
            }

            // freturn
            TokenKind::TFreturn => {
                self.advance();
                JvmInstruction::Freturn
            }

            // fstore <varnum>
            TokenKind::TFstore => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Fstore { varnum }
            }

            // fstore_0
            TokenKind::TFstore0 => {
                self.advance();
                JvmInstruction::Fstore0
            }

            // fstore_1
            TokenKind::TFstore1 => {
                self.advance();
                JvmInstruction::Fstore1
            }

            // fstore_2
            TokenKind::TFstore2 => {
                self.advance();
                JvmInstruction::Fstore2
            }

            // fstore_3
            TokenKind::TFstore3 => {
                self.advance();
                JvmInstruction::Fstore3
            }

            // fsub
            TokenKind::TFsub => {
                self.advance();
                JvmInstruction::Fsub
            }

            // getfield <field-spec> <descriptor>
            TokenKind::TGetfield => {
                let start_span = self.curr_span();

                self.advance();

                if let TokenKind::TIdent(ref gf_str) = self.see().kind {
                    if let Some(pos) = gf_str.rfind('/') {
                        let class_name = gf_str[..pos].to_owned();
                        let field_name = gf_str[pos + 1..].to_owned();

                        self.advance();

                        let field_descriptor = self.parse_field_descriptor().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                start_span.merge(&self.curr_span()),
                                format!("missing field descriptor"),
                            );
                            self.errored |= true;

                            Some(PhoronFieldDescriptor::default())
                        })?;

                        JvmInstruction::Getfield {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            start_span.merge(&self.curr_span()),
                            format!("missing field name"),
                        );
                        self.errored |= true;

                        JvmInstruction::Getfield {
                            class_name: String::default(),
                            field_name: String::default(),
                            field_descriptor: PhoronFieldDescriptor::default(),
                        }
                    }
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing class name"),
                    );
                    self.errored |= true;

                    JvmInstruction::Getfield {
                        class_name: String::default(),
                        field_name: String::default(),
                        field_descriptor: PhoronFieldDescriptor::default(),
                    }
                }
            }

            // getstatic <field-spec> <descriptor>
            TokenKind::TGetstatic => {
                let start_span = self.curr_span();

                self.advance();

                if let TokenKind::TIdent(ref gs_str) = self.see().kind {
                    if let Some(pos) = gs_str.rfind('/') {
                        let class_name = gs_str[..pos].to_owned();
                        let field_name = gs_str[pos + 1..].to_owned();

                        self.advance();

                        let field_descriptor = self.parse_field_descriptor().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                start_span.merge(&self.curr_span()),
                                format!("missing field descriptor"),
                            );
                            self.errored |= true;

                            Some(PhoronFieldDescriptor::default())
                        })?;

                        JvmInstruction::Getstatic {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            start_span.merge(&self.curr_span()),
                            format!("missing field name"),
                        );

                        self.errored |= true;

                        JvmInstruction::Getstatic {
                            class_name: String::default(),
                            field_name: String::default(),
                            field_descriptor: PhoronFieldDescriptor::default(),
                        }
                    }
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing class name"),
                    );
                    self.errored |= true;

                    JvmInstruction::Getstatic {
                        class_name: String::default(),
                        field_name: String::default(),
                        field_descriptor: PhoronFieldDescriptor::default(),
                    }
                }
            }

            // goto <label>
            TokenKind::TGoto => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Goto { label }
            }

            // goto_w <label>
            TokenKind::TGotow => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Gotow { label }
            }

            // i2b
            TokenKind::TI2b => {
                self.advance();
                JvmInstruction::I2b
            }

            // i2c
            TokenKind::TI2c => {
                self.advance();
                JvmInstruction::I2c
            }

            // i2d
            TokenKind::TI2d => {
                self.advance();
                JvmInstruction::I2c
            }

            // i2f
            TokenKind::TI2f => {
                self.advance();
                JvmInstruction::I2f
            }

            // i2l
            TokenKind::TI2l => {
                self.advance();
                JvmInstruction::I2l
            }

            // i2s
            TokenKind::TI2s => {
                self.advance();
                JvmInstruction::I2s
            }

            // iadd
            TokenKind::TIadd => {
                self.advance();
                JvmInstruction::Iadd
            }

            // iaload
            TokenKind::TIaload => {
                self.advance();
                JvmInstruction::Iaload
            }

            // iand
            TokenKind::TIand => {
                self.advance();
                JvmInstruction::Iand
            }

            // iastore
            TokenKind::TIastore => {
                self.advance();
                JvmInstruction::Iastore
            }

            // iconst_m1
            TokenKind::TIconstm1 => {
                self.advance();
                JvmInstruction::Iconstm1
            }

            // iconst_0
            TokenKind::TIconst0 => {
                self.advance();
                JvmInstruction::Iconst0
            }

            // iconst_1
            TokenKind::TIconst1 => {
                self.advance();
                JvmInstruction::Iconst1
            }

            // iconst_2
            TokenKind::TIconst2 => {
                self.advance();
                JvmInstruction::Iconst2
            }

            // iconst-3
            TokenKind::TIconst3 => {
                self.advance();
                JvmInstruction::Iconst3
            }

            // iconst_4
            TokenKind::TIconst4 => {
                self.advance();
                JvmInstruction::Iconst4
            }

            // iconst_5
            TokenKind::TIconst5 => {
                self.advance();
                JvmInstruction::Iconst5
            }

            // idiv
            TokenKind::TIdiv => {
                self.advance();
                JvmInstruction::Idiv
            }

            // if_acmpeq <label>
            TokenKind::TIfacmpeq => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ifacmpeq { label }
            }

            // if_acmpne <label>
            TokenKind::TIfacmpne => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ifacmpne { label }
            }

            // if_icmpeq <label>
            TokenKind::TIficmpeq => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ificmpeq { label }
            }

            // if_icmpge <label>
            TokenKind::TIficmpge => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ificmpge { label }
            }

            // if_icmpgt <label>
            TokenKind::TIficmpgt => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ificmpgt { label }
            }

            // if_icmple <label>
            TokenKind::TIficmple => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ificmple { label }
            }

            // if_icmplt <label>
            TokenKind::TIficmplt => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ificmplt { label }
            }

            // ifne <label>
            TokenKind::TIfne => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ifne { label }
            }

            // if_icmpne <label>
            TokenKind::TIficmpne => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ificmpne { label }
            }

            // ifeq <label>
            TokenKind::TIfeq => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ifeq { label }
            }

            // ifge <label>
            TokenKind::TIfge => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ifge { label }
            }

            // ifgt <label>
            TokenKind::TIfgt => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ifgt { label }
            }

            // ifle <label>
            TokenKind::TIfle => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ifle { label }
            }

            // iflt <label>
            TokenKind::TIflt => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Iflt { label }
            }

            // ifnonnull <label>
            TokenKind::TIfnonnull => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ifnonnull { label }
            }

            // ifnull <label>
            TokenKind::TIfnull => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Ifnull { label }
            }

            // iinc <varnum> <n>
            TokenKind::TIinc => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                let delta = self.parse_sb().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing delta"),
                    );
                    self.errored |= true;

                    Some(i8::default())
                })?;

                JvmInstruction::Iinc { varnum, delta }
            }

            // iload <varnum>
            TokenKind::TIload => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Iload { varnum }
            }

            // iload_0
            TokenKind::TIload0 => {
                self.advance();
                JvmInstruction::Iload0
            }

            // iload_1
            TokenKind::TIload1 => {
                self.advance();
                JvmInstruction::Iload1
            }

            // iload_2
            TokenKind::TIload2 => {
                self.advance();
                JvmInstruction::Iload2
            }

            // iload_3
            TokenKind::TIload3 => {
                self.advance();
                JvmInstruction::Iload3
            }

            // imul
            TokenKind::TImul => {
                self.advance();
                JvmInstruction::Imul
            }

            // ineg
            TokenKind::TIneg => {
                self.advance();
                JvmInstruction::Ineg
            }

            // instanceof
            TokenKind::TInstanceof => {
                self.advance();

                let check_type = self.parse_field_descriptor().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing check type"),
                    );
                    self.errored |= true;

                    Some(PhoronFieldDescriptor::default())
                })?;

                JvmInstruction::Instanceof { check_type }
            }

            // invokeinterface <method-spec> <n>
            TokenKind::TInvokeinterface => {
                let start_span = self.curr_span();

                self.advance();

                if let TokenKind::TIdent(ref is_str) = self.see().kind {
                    if let Some(pos) = is_str.rfind('/') {
                        let interface_name = is_str[..pos].to_owned();
                        let method_name = is_str[pos + 1..].to_owned();

                        self.advance();

                        let ub = self.parse_ub().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                start_span.merge(&self.curr_span()),
                                format!("missing unsigned byte constant"),
                            );
                            self.errored |= true;

                            Some(u8::default())
                        })?;

                        let method_descriptor = self.parse_method_descriptor().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                start_span.merge(&self.curr_span()),
                                format!("missing method descriptor"),
                            );
                            self.errored |= true;

                            Some(PhoronMethodDescriptor::default())
                        })?;

                        JvmInstruction::Invokeinterface {
                            interface_name,
                            method_name,
                            method_descriptor,
                            ub,
                        }
                    } else {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            start_span.merge(&self.curr_span()),
                            format!("missing method name"),
                        );
                        self.errored |= true;

                        JvmInstruction::Invokeinterface {
                            interface_name: String::default(),
                            method_name: String::default(),
                            method_descriptor: PhoronMethodDescriptor::default(),
                            ub: u8::default(),
                        }
                    }
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing class name"),
                    );
                    self.errored |= true;

                    JvmInstruction::Invokeinterface {
                        interface_name: String::default(),
                        method_name: String::default(),
                        method_descriptor: PhoronMethodDescriptor::default(),
                        ub: u8::default(),
                    }
                }
            }

            // invokespecial <method-spec>
            TokenKind::TInvokespecial => {
                let start_span = self.curr_span();

                self.advance();

                if let TokenKind::TIdent(ref is_str) = self.see().kind {
                    if let Some(pos) = is_str.rfind('/') {
                        let class_name = is_str[..pos].to_owned();
                        let method_name = is_str[pos + 1..].to_owned();

                        self.advance();

                        let method_descriptor = self.parse_method_descriptor().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                start_span.merge(&self.curr_span()),
                                format!("missing method descriptor"),
                            );
                            self.errored |= true;

                            Some(PhoronMethodDescriptor::default())
                        })?;

                        JvmInstruction::Invokespecial {
                            class_name,
                            method_name,
                            method_descriptor,
                        }
                    } else {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            start_span.merge(&self.curr_span()),
                            format!("missing method name"),
                        );
                        self.errored |= true;

                        JvmInstruction::Invokespecial {
                            class_name: String::default(),
                            method_name: String::default(),
                            method_descriptor: PhoronMethodDescriptor::default(),
                        }
                    }
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing class name"),
                    );
                    self.errored |= true;

                    JvmInstruction::Invokespecial {
                        class_name: String::default(),
                        method_name: String::default(),
                        method_descriptor: PhoronMethodDescriptor::default(),
                    }
                }
            }

            // invokestatic <method-spec>
            TokenKind::TInvokestatic => {
                let start_span = self.curr_span();

                self.advance();

                if let TokenKind::TIdent(ref is_str) = self.see().kind {
                    if let Some(pos) = is_str.rfind('/') {
                        let class_name = is_str[..pos].to_owned();
                        let method_name = is_str[pos + 1..].to_owned();

                        self.advance();

                        let method_descriptor = self.parse_method_descriptor().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                start_span.merge(&self.curr_span()),
                                format!("missing method descriptor"),
                            );
                            self.errored |= true;

                            Some(PhoronMethodDescriptor::default())
                        })?;

                        JvmInstruction::Invokestatic {
                            class_name,
                            method_name,
                            method_descriptor,
                        }
                    } else {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            start_span.merge(&self.curr_span()),
                            format!("missing method name"),
                        );
                        self.errored |= true;

                        JvmInstruction::Invokestatic {
                            class_name: String::default(),
                            method_name: String::default(),
                            method_descriptor: PhoronMethodDescriptor::default(),
                        }
                    }
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing class name"),
                    );
                    self.errored |= true;

                    JvmInstruction::Invokestatic {
                        class_name: String::default(),
                        method_name: String::default(),
                        method_descriptor: PhoronMethodDescriptor::default(),
                    }
                }
            }

            // invokevirtual <method-spec>
            TokenKind::TInvokevirtual => {
                let start_span = self.curr_span();

                self.advance();

                if let TokenKind::TIdent(ref is_str) = self.see().kind {
                    if let Some(pos) = is_str.rfind('/') {
                        let class_name = is_str[..pos].to_owned();
                        let method_name = is_str[pos + 1..].to_owned();

                        self.advance();

                        let method_descriptor = self.parse_method_descriptor().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                start_span.merge(&self.curr_span()),
                                format!("missing method descriptor"),
                            );
                            self.errored |= true;

                            Some(PhoronMethodDescriptor::default())
                        })?;

                        JvmInstruction::Invokevirtual {
                            class_name,
                            method_name,
                            method_descriptor,
                        }
                    } else {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            start_span.merge(&self.curr_span()),
                            format!("missing method name"),
                        );
                        self.errored |= true;

                        JvmInstruction::Invokevirtual {
                            class_name: String::default(),
                            method_name: String::default(),
                            method_descriptor: PhoronMethodDescriptor::default(),
                        }
                    }
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing class name"),
                    );

                    self.errored |= true;

                    JvmInstruction::Invokevirtual {
                        class_name: String::default(),
                        method_name: String::default(),
                        method_descriptor: PhoronMethodDescriptor::default(),
                    }
                }
            }

            // ior
            TokenKind::TIor => {
                self.advance();
                JvmInstruction::Ior
            }

            // irem
            TokenKind::TIrem => {
                self.advance();
                JvmInstruction::Irem
            }

            // ireturn
            TokenKind::TIreturn => {
                self.advance();
                JvmInstruction::Ireturn
            }

            // ishl
            TokenKind::TIshl => {
                self.advance();
                JvmInstruction::Ishl
            }

            // lshr
            TokenKind::TIshr => {
                self.advance();
                JvmInstruction::Ishr
            }

            // istore <varnum>
            TokenKind::TIstore => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Istore { varnum }
            }

            // istore_0
            TokenKind::TIstore0 => {
                self.advance();
                JvmInstruction::Istore0
            }

            // istore_1
            TokenKind::TIstore1 => {
                self.advance();
                JvmInstruction::Istore1
            }

            // istore_2
            TokenKind::TIstore2 => {
                self.advance();
                JvmInstruction::Istore2
            }

            // istore_3
            TokenKind::TIstore3 => {
                self.advance();
                JvmInstruction::Istore3
            }

            // isub
            TokenKind::TIsub => {
                self.advance();
                JvmInstruction::Isub
            }

            // iushr
            TokenKind::TIushr => {
                self.advance();
                JvmInstruction::Iushr
            }

            // ixor
            TokenKind::TIxor => {
                self.advance();
                JvmInstruction::Ixor
            }

            // jsr <label>
            TokenKind::TJsr => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Jsr { label }
            }

            // jsr_w <label>
            TokenKind::TJsrw => {
                self.advance();

                let label = self.parse_label().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing label"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Jsrw { label }
            }

            // l2d
            TokenKind::TL2d => {
                self.advance();
                JvmInstruction::L2d
            }

            // l2f
            TokenKind::TL2f => {
                self.advance();
                JvmInstruction::L2f
            }

            // l2i
            TokenKind::TL2i => {
                self.advance();
                JvmInstruction::L2i
            }

            // ladd
            TokenKind::TLadd => {
                self.advance();
                JvmInstruction::Ladd
            }

            // laload
            TokenKind::TLaload => {
                self.advance();
                JvmInstruction::Laload
            }

            // land
            TokenKind::TLand => {
                self.advance();
                JvmInstruction::Land
            }

            // lastore
            TokenKind::TLastore => {
                self.advance();
                JvmInstruction::Lastore
            }

            // lcmp
            TokenKind::TLcmp => {
                self.advance();
                JvmInstruction::Lcmp
            }

            // locaonst_0
            TokenKind::TLconst0 => {
                self.advance();
                JvmInstruction::Lconst0
            }

            // lconst_1
            TokenKind::TLconst1 => {
                self.advance();
                JvmInstruction::Lconst1
            }

            // ldc <integer / float / quoted string>
            TokenKind::TLdc => {
                self.advance();

                match &self.see().kind {
                    TokenKind::TInt(n) => {
                        let ival = *n as i32;
                        self.advance();
                        JvmInstruction::Ldc(LdcValue::Integer(ival))
                    }

                    TokenKind::TFloat(f) => {
                        let fval = *f as f32;
                        self.advance();
                        JvmInstruction::Ldc(LdcValue::Float(fval))
                    }

                    TokenKind::TString(ref s) => {
                        let sval = s.to_owned();
                        self.advance();
                        JvmInstruction::Ldc(LdcValue::QuotedString(sval))
                    }

                    tok_kind => {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            self.curr_span(),
                            format!("found {tok_kind:?}, but I expected an int, float, or string value here")
                        );
                        self.errored |= true;

                        JvmInstruction::Ldc(LdcValue::default())
                    }
                }
            }

            // ldcw <integer / float / quoted string>
            TokenKind::TLdcw => {
                self.advance();

                match &self.see().kind {
                    TokenKind::TInt(n) => {
                        let ival = *n as i32;
                        self.advance();
                        JvmInstruction::Ldcw(LdcwValue::Integer(ival))
                    }

                    TokenKind::TFloat(f) => {
                        let fval = *f as f32;
                        self.advance();
                        JvmInstruction::Ldcw(LdcwValue::Float(fval))
                    }

                    TokenKind::TString(s) => {
                        let sval = s.to_owned();
                        self.advance();
                        JvmInstruction::Ldcw(LdcwValue::QuotedString(sval))
                    }

                    tok_kind => {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            self.curr_span(),
                            format!(
                                "found {tok_kind:?}, but I expected an int, float, or string here"
                            ),
                        );
                        self.errored |= true;

                        JvmInstruction::Ldcw(LdcwValue::default())
                    }
                }
            }

            // ldc2_w <Long / Double>
            TokenKind::TLdc2w => {
                self.advance();

                match &self.see().kind {
                    TokenKind::TInt(n) => {
                        let lval = *n as i64;
                        self.advance();
                        JvmInstruction::Ldc2w(Ldc2wValue::Long(lval))
                    }

                    TokenKind::TFloat(f) => {
                        let dval = *f as f64;
                        self.advance();
                        JvmInstruction::Ldc2w(Ldc2wValue::Double(dval))
                    }

                    tok_kind => {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            self.curr_span(),
                            format!("found {tok_kind:?}, but I expected a long or double here"),
                        );
                        self.errored |= true;

                        JvmInstruction::Ldc2w(Ldc2wValue::default())
                    }
                }
            }

            // ldiv
            TokenKind::TLdiv => {
                self.advance();
                JvmInstruction::Ldiv
            }

            // lload <varnum>
            TokenKind::TLload => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Lload { varnum }
            }

            // lload_0
            TokenKind::TLload0 => {
                self.advance();
                JvmInstruction::Lload0
            }

            // lload_1
            TokenKind::TLload1 => {
                self.advance();
                JvmInstruction::Lload1
            }

            // lload_2
            TokenKind::TLload2 => {
                self.advance();
                JvmInstruction::Lload2
            }

            //lload_3
            TokenKind::TLload3 => {
                self.advance();
                JvmInstruction::Lload3
            }

            // lmul
            TokenKind::TLmul => {
                self.advance();
                JvmInstruction::Lmul
            }

            // lneg
            TokenKind::TLneg => {
                self.advance();
                JvmInstruction::Lneg
            }

            // lookupswitch          <-  'lookupswitch'   LookupSwitchPair*  DefaultSwitchPair
            // LookupSwitchPair      <-  Integer          COLON_symbol       Label
            // DefaultSwitchPair     <-  DEFAULT_keyword  COLON_symbol       Label
            TokenKind::TLookupswitch => {
                self.advance();

                let mut switches = self.parse_lookup_switches()?;
                // sort the switches in order of keys
                switches.sort_by_key(|p| p.key);

                let default = self.parse_default_switch_pair()?;

                JvmInstruction::Lookupswitch { switches, default }
            }

            // lor
            TokenKind::TLor => {
                self.advance();
                JvmInstruction::Lor
            }

            // lrem
            TokenKind::TLrem => {
                self.advance();
                JvmInstruction::Lrem
            }

            // lreturn
            TokenKind::TLreturn => {
                self.advance();
                JvmInstruction::Lreturn
            }

            // lshl
            TokenKind::TLshl => {
                self.advance();
                JvmInstruction::Lshl
            }

            // lshr
            TokenKind::TLshr => {
                self.advance();
                JvmInstruction::Lshr
            }

            // lstore <varnum>
            TokenKind::TLstore => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Lstore { varnum }
            }

            // lstore_0
            TokenKind::TLstore0 => {
                self.advance();
                JvmInstruction::Lstore0
            }

            // lstore_1
            TokenKind::TLstore1 => {
                self.advance();
                JvmInstruction::Lstore1
            }

            // lstore_2
            TokenKind::TLstore2 => {
                self.advance();
                JvmInstruction::Lstore2
            }

            // lstore_3
            TokenKind::TLstore3 => {
                self.advance();
                JvmInstruction::Lstore3
            }

            // lsub
            TokenKind::TLsub => {
                self.advance();
                JvmInstruction::Lsub
            }

            // lushr
            TokenKind::TLushr => {
                self.advance();
                JvmInstruction::Lushr
            }

            // lxor
            TokenKind::TLxor => {
                self.advance();
                JvmInstruction::Lxor
            }

            // monitorenter
            TokenKind::TMonitorenter => {
                self.advance();
                JvmInstruction::Monitorenter
            }

            // monitorexit
            TokenKind::TMonitorexit => {
                self.advance();
                JvmInstruction::Monitorexit
            }

            TokenKind::TMultianewarray => {
                let start_span = self.curr_span();

                self.advance();

                let component_type = self.parse_field_descriptor().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing component type"),
                    );
                    self.errored |= true;

                    Some(PhoronFieldDescriptor::default())
                })?;

                let dimensions = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing dimensions"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Multianewarray {
                    component_type,
                    dimensions,
                }
            }

            // new <class>
            TokenKind::TNew => {
                self.advance();

                let class_name = self.parse_class_name().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing class name"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::New { class_name }
            }

            // newarray <primitive_type>
            TokenKind::TNewarray => {
                self.advance();

                if let TokenKind::TIdent(ref prim_type) = self.see().kind {
                    match prim_type.as_str() {
                        "char" => {
                            self.advance();
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Character,
                            }
                        }

                        "float" => {
                            self.advance();
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Float,
                            }
                        }

                        "double" => {
                            self.advance();
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Double,
                            }
                        }

                        "byte" => {
                            self.advance();
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Byte,
                            }
                        }

                        "short" => {
                            self.advance();
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Short,
                            }
                        }

                        "int" => {
                            self.advance();
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Integer,
                            }
                        }

                        "long" => {
                            self.advance();
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Long,
                            }
                        }

                        tok_kind => {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!(
                                    "{tok_kind:?} is not a primitive type, which I expected here"
                                ),
                            );
                            self.errored |= true;

                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::default(),
                            }
                        }
                    }
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("malformed instruction"),
                    );
                    self.errored |= true;

                    JvmInstruction::Newarray {
                        component_type: PhoronBaseType::default(),
                    }
                }
            }

            // nop
            TokenKind::TNop => {
                self.advance();
                JvmInstruction::Nop
            }

            // pop
            TokenKind::TPop => {
                self.advance();
                JvmInstruction::Pop
            }

            // pop2
            TokenKind::TPop2 => {
                self.advance();
                JvmInstruction::Pop2
            }

            // putfield <field-sepc> <descriptor>
            TokenKind::TPutfield => {
                let start_span = self.curr_span();

                self.advance();

                if let TokenKind::TIdent(ref gf_str) = self.see().kind {
                    if let Some(pos) = gf_str.rfind('/') {
                        let class_name = gf_str[..pos].to_owned();
                        let field_name = gf_str[pos + 1..].to_owned();

                        self.advance();

                        let field_descriptor = self.parse_field_descriptor().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                start_span.merge(&self.curr_span()),
                                format!("missing field descriptor"),
                            );
                            self.errored |= true;

                            Some(PhoronFieldDescriptor::default())
                        })?;

                        JvmInstruction::Putfield {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            start_span.merge(&self.curr_span()),
                            format!("missing field name"),
                        );
                        self.errored |= true;

                        JvmInstruction::Putfield {
                            class_name: String::default(),
                            field_name: String::default(),
                            field_descriptor: PhoronFieldDescriptor::default(),
                        }
                    }
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing class name"),
                    );
                    self.errored |= true;

                    JvmInstruction::Putfield {
                        class_name: String::default(),
                        field_name: String::default(),
                        field_descriptor: PhoronFieldDescriptor::default(),
                    }
                }
            }

            // putstatic <field-spec> <descriptor>
            TokenKind::TPutstatic => {
                let start_span = self.curr_span();

                self.advance();

                if let TokenKind::TIdent(ref gf_str) = self.see().kind {
                    if let Some(pos) = gf_str.rfind('/') {
                        let class_name = gf_str[..pos].to_owned();
                        let field_name = gf_str[pos + 1..].to_owned();

                        self.advance();

                        let field_descriptor = self.parse_field_descriptor().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                start_span.merge(&self.curr_span()),
                                format!("missing field descriptor"),
                            );
                            self.errored |= true;

                            Some(PhoronFieldDescriptor::default())
                        })?;

                        JvmInstruction::Putstatic {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            start_span.merge(&self.curr_span()),
                            format!("missing field name"),
                        );
                        self.errored |= true;

                        JvmInstruction::Putstatic {
                            class_name: String::default(),
                            field_name: String::default(),
                            field_descriptor: PhoronFieldDescriptor::default(),
                        }
                    }
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing class name"),
                    );
                    self.errored |= true;

                    JvmInstruction::Putstatic {
                        class_name: String::default(),
                        field_name: String::default(),
                        field_descriptor: PhoronFieldDescriptor::default(),
                    }
                }
            }

            // ret <varnum>
            TokenKind::TRet => {
                self.advance();

                let varnum = self.parse_ub().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing var num"),
                    );
                    self.errored |= true;

                    Some(u8::default())
                })?;

                JvmInstruction::Ret { varnum }
            }

            // return
            TokenKind::TReturn => {
                self.advance();
                JvmInstruction::Return
            }

            // saload
            TokenKind::TSaload => {
                self.advance();
                JvmInstruction::Saload
            }

            // sastore
            TokenKind::TSastore => {
                self.advance();
                JvmInstruction::Sastore
            }

            // sipush i16
            TokenKind::TSipush => {
                self.advance();

                let ss = self.parse_ss().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing signed 16-byte constant"),
                    );
                    self.errored |= true;

                    Some(i16::default())
                })?;

                JvmInstruction::Sipush(ss)
            }

            // swap
            TokenKind::TSwap => {
                self.advance();
                JvmInstruction::Swap
            }

            // tableswitch    <-  'tableswitch'   Low   High  TableSwitchSingleton*  DefaultSwitchPair
            // TableSwitchSingleton  <-  Label
            TokenKind::TTableswitch => {
                let start_span = self.curr_span();

                self.advance();

                let low = self.parse_si().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `low` value"),
                    );
                    self.errored |= true;

                    Some(i32::default())
                })?;

                let high = self.parse_si().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing `high` value"),
                    );
                    self.errored |= true;

                    Some(i32::default())
                })?;

                let switches = self.parse_table_switches()?;

                let default = self.parse_default_switch_pair().or_else(|| {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing default case"),
                    );
                    self.errored |= true;

                    Some(String::default())
                })?;

                JvmInstruction::Tableswitch {
                    low,
                    high,
                    switches,
                    default,
                }
            }

            // The `wide` instruction must be followed by one of the
            // following instructions:
            // iload, fload, aload, lload, dload, istore, fstore, astore,
            // lstore, dstore, or iinc.
            TokenKind::Twide => {
                self.advance();

                match &self.see().kind {
                    TokenKind::TIload => {
                        self.advance();

                        let varnum = self.parse_us().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing var num"),
                            );
                            self.errored |= true;

                            Some(u16::default())
                        })?;

                        JvmInstruction::Wide(WideInstruction::Iload { varnum })
                    }

                    TokenKind::TFload => {
                        self.advance();

                        let varnum = self.parse_us().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing var num"),
                            );
                            self.errored |= true;

                            Some(u16::default())
                        })?;

                        JvmInstruction::Wide(WideInstruction::Fload { varnum })
                    }

                    TokenKind::TAload => {
                        self.advance();

                        let varnum = self.parse_us().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing var num"),
                            );
                            self.errored |= true;

                            Some(u16::default())
                        })?;

                        JvmInstruction::Wide(WideInstruction::Aload { varnum })
                    }

                    TokenKind::TLload => {
                        self.advance();

                        let varnum = self.parse_us().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing var num"),
                            );
                            self.errored |= true;

                            Some(u16::default())
                        })?;

                        JvmInstruction::Wide(WideInstruction::Lload { varnum })
                    }

                    TokenKind::TDload => {
                        self.advance();

                        let varnum = self.parse_us().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing var num"),
                            );
                            self.errored |= true;

                            Some(u16::default())
                        })?;

                        JvmInstruction::Wide(WideInstruction::Dload { varnum })
                    }

                    TokenKind::TIstore => {
                        self.advance();

                        let varnum = self.parse_us().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing var num"),
                            );
                            self.errored |= true;

                            Some(u16::default())
                        })?;

                        JvmInstruction::Wide(WideInstruction::Istore { varnum })
                    }

                    TokenKind::TFstore => {
                        self.advance();

                        let varnum = self.parse_us().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing var num"),
                            );
                            self.errored |= true;

                            Some(u16::default())
                        })?;

                        JvmInstruction::Wide(WideInstruction::Fstore { varnum })
                    }

                    TokenKind::TAstore => {
                        self.advance();

                        let varnum = self.parse_us().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing var num"),
                            );
                            self.errored |= true;

                            Some(u16::default())
                        })?;

                        JvmInstruction::Wide(WideInstruction::Astore { varnum })
                    }

                    TokenKind::TLstore => {
                        self.advance();

                        let varnum = self.parse_us().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing var num"),
                            );
                            self.errored |= true;

                            Some(u16::default())
                        })?;

                        JvmInstruction::Wide(WideInstruction::Lstore { varnum })
                    }

                    TokenKind::TDstore => {
                        self.advance();

                        let varnum = self.parse_us().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing var num"),
                            );
                            self.errored |= true;

                            Some(u16::default())
                        })?;

                        JvmInstruction::Wide(WideInstruction::Dstore { varnum })
                    }

                    TokenKind::TIinc => {
                        self.advance();

                        let varnum = self.parse_us().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing var num"),
                            );
                            self.errored |= true;

                            Some(u16::default())
                        })?;

                        let delta = self.parse_ss().or_else(|| {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                self.curr_span(),
                                format!("missing delta"),
                            );
                            self.errored |= true;

                            Some(i16::default())
                        })?;

                        JvmInstruction::Wide(WideInstruction::IInc { varnum, delta })
                    }

                    instr => {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            self.curr_span(),
                            format!(
                                "{instr:?} - incorrect instruction following `wide` instruction"
                            ),
                        );
                        self.errored |= true;

                        JvmInstruction::default()
                    }
                }
            }

            _ => {
                unreachable!()
            }
        })
    }

    /// Instruction <- line_comment* (Directive / JvmInstruction / Label) line_comment?  newline
    fn parse_instruction(&mut self) -> Option<PhoronInstruction> {
        Some(match &self.see().kind {
            TThrows | TCatch | TLimit | TVar | TLine => {
                PhoronInstruction::PhoronDirective(self.parse_directive()?)
            }

            TAaload | TAastore | TAconstnull | TAload | TAload0 | TAload1 | TAload2 | TAload3
            | TAnewarray | TAreturn | TArraylength | TAssign | TAstore | TAstore0 | TAstore1
            | TAstore2 | TAstore3 | TAthrow | TBaload | TBastore | TBipush | TBridge | TCaload
            | TCastore | TCheckcast | TD2f | TD2i | TD2l | TDadd | TDaload | TDastore | TDcmpg
            | TDcmpl | TDconst0 | TDconst1 | TDdiv | TDefault | TDload | TDload0 | TDload1
            | TDload2 | TDload3 | TDmul | TDneg | TDot | TDrem | TDreturn | TDstore | TDstore0
            | TDstore1 | TDstore2 | TDstore3 | TDsub | TDup | TDup2 | TDup2x1 | TDup2x2
            | TDupx1 | TDupx2 | TEnum | TF2d | TF2i | TF2l | TFadd | TFaload | TFastore
            | TFcmpg | TFcmpl | TFconst0 | TFconst1 | TFconst2 | TFdiv | TField | TFinal
            | TFload | TFload0 | TFload1 | TFload2 | TFload3 | TFmul | TFneg | TFrem | TFreturn
            | TFrom | TFstore | TFstore0 | TFstore1 | TFstore2 | TFstore3 | TFsub | TGetfield
            | TGetstatic | TGoto | TGotow | TI2b | TI2c | TI2d | TI2f | TI2l | TI2s | TIadd
            | TIaload | TIand | TIastore | TIconst0 | TIconst1 | TIconst2 | TIconst3 | TIconst4
            | TIconst5 | TIconstm1 | TIdiv | TIfacmpeq | TIfacmpne | TIfeq | TIfge | TIfgt
            | TIficmpeq | TIficmpge | TIficmpgt | TIficmple | TIficmplt | TIficmpne | TIfle
            | TIflt | TIfne | TIfnonnull | TIfnull | TIinc | TIload | TIload0 | TIload1
            | TIload2 | TIload3 | TImul | TIneg | TInstanceof | TInvokeinterface
            | TInvokespecial | TInvokestatic | TInvokevirtual | TIor | TIrem | TIreturn | TIshl
            | TIshr | TIstore | TIstore0 | TIstore1 | TIstore2 | TIstore3 | TIsub | TIushr
            | TIxor | TJsr | TJsrw | TL2d | TL2f | TL2i | TLadd | TLand | TLastore | TLcmp
            | TLconst0 | TLconst1 | TLdc | TLdc2w | TLdcw | TLdiv | TLload | TLload0 | TLload1
            | TLload2 | TLload3 | TLmul | TLneg | TLoaload | TLookupswitch | TLor | TLrem
            | TLreturn | TLshl | TLshr | TLstore | TLstore0 | TLstore1 | TLstore2 | TLstore3
            | TLsub | TLushr | TLxor | TMonitorenter | TMonitorexit | TMultianewarray | TNew
            | TNewarray | TNop | TPop | TPop2 | TPutfield | TPutstatic | TRet | TReturn
            | TSaload | TSastore | TSipush | TSuper | TSwap | TTableswitch => {
                PhoronInstruction::JvmInstruction(self.parse_jvm_instruction()?)
            }

            TokenKind::TIdent(label_str) => {
                let label = label_str.to_string();
                self.advance();

                if let Token {
                    kind: TokenKind::TColon,
                    ..
                } = self.see()
                {
                    self.advance();
                    PhoronInstruction::PhoronLabel(label)
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("unknown instruction or label"),
                    );

                    if self.see().kind != TokenKind::TEnd {
                        self.advance();
                    }

                    self.errored |= true;

                    PhoronInstruction::default()
                }
            }

            _ => {
                if self.see().kind == TokenKind::TEnd {
                    PhoronInstruction::default()
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("{:?} unknown instruction", self.see().kind),
                    );

                    self.advance();
                    self.errored |= true;

                    PhoronInstruction::default()
                }
            }
        })
    }

    fn parse_instructions(&mut self) -> Option<Vec<PhoronInstruction>> {
        let mut instructions = Vec::new();

        while self.see().kind != TokenKind::TEof {
            if let Token {
                kind: TokenKind::TEnd,
                span: _,
            } = self.see()
            {
                self.advance();

                if let TokenKind::TEndMethod = self.see().kind {
                    self.advance();
                    break;
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        self.curr_span(),
                        format!("missing end method marker"),
                    );

                    self.advance();
                    self.errored |= true;

                    return Some(vec![]);
                }
            } else {
                instructions.push(self.parse_instruction()?);
            }
        }

        Some(instructions)
    }

    /// MethodDescriptor <- LPAREN_symbol ParameterDescriptor* RPAREN_symbol ReturnDescriptor
    /// ParameterDescriptor <- FieldType
    /// ReturnDescriptor <- FieldType / VoidType
    /// VoidType <- 'V'
    fn parse_method_descriptor(&mut self) -> Option<PhoronMethodDescriptor> {
        let start_span = self.curr_span();

        if let TokenKind::TLeftParen = self.see().kind {
            self.advance();

            let ident_tok = self.see();
            let param_descriptor = if let Token {
                kind: TokenKind::TIdent(ident),
                ..
            } = ident_tok
            {
                let mut param_parser = tdp::TypeParser::new(ident);

                let param_desc = match param_parser.parse_param_descriptor() {
                    Err(_err) => {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            start_span.merge(&self.curr_span()),
                            format!("missing param descriptor"),
                        );

                        self.errored |= true;

                        vec![]
                    }
                    Ok(param_desc) => param_desc,
                };

                self.advance();

                param_desc
            } else {
                vec![]
            };

            if let Token {
                kind: TokenKind::TRightParen,
                ..
            } = self.see()
            {
                self.advance();

                let ident_tok = self.see();
                if let Token {
                    kind: TokenKind::TIdent(ret),
                    ..
                } = ident_tok
                {
                    let mut ret_parser = tdp::TypeParser::new(ret);
                    let return_descriptor = match ret_parser.parse_return_descriptor() {
                        Err(_err) => {
                            DiagnosticManager::report_diagnostic(
                                &self.lexer.source_file,
                                start_span.merge(&self.curr_span()),
                                format!("missing return descriptor"),
                            );

                            self.errored |= true;

                            PhoronReturnDescriptor::default()
                        }
                        Ok(ret_desc) => ret_desc,
                    };

                    self.advance();

                    Some(PhoronMethodDescriptor {
                        param_descriptor,
                        return_descriptor,
                    })
                } else {
                    DiagnosticManager::report_diagnostic(
                        &self.lexer.source_file,
                        start_span.merge(&self.curr_span()),
                        format!("missing return descriptor"),
                    );

                    self.advance();
                    self.errored |= true;

                    None
                }
            } else {
                DiagnosticManager::report_diagnostic(
                    &self.lexer.source_file,
                    start_span.merge(&self.curr_span()),
                    format!("malformed return descriptorname"),
                );

                self.advance();
                self.errored |= true;

                None
            }
        } else {
            DiagnosticManager::report_diagnostic(
                &self.lexer.source_file,
                start_span.merge(&self.curr_span()),
                format!("malformed return descriptor name"),
            );

            self.advance();
            self.errored |= true;

            None
        }
    }

    /// MethodDef <- line_comment*
    ///    METHOD_keyword  MethodAccessFlag* MethodName MethodDescriptor newline
    ///      Instruction*
    ///    END_Keyword METHOD_END_keyword newline
    fn parse_method_def(&mut self) -> Option<PhoronMethodDef> {
        let start_span = self.curr_span();

        self.advance();

        let mut access_flags = Vec::new();
        while self.is_method_access_flag(&self.see().kind) {
            access_flags.push(self.get_method_acess_flags(&self.see().kind));
            self.advance();
        }

        if let Token {
            kind: TokenKind::TIdent(name_str),
            span: _,
        } = self.see()
        {
            let name = name_str.to_string();
            self.advance();

            let method_descriptor = self
                .parse_method_descriptor()
                .or(Some(PhoronMethodDescriptor::default()))?;

            let instructions = self.parse_instructions().or(Some(vec![]))?;

            Some(PhoronMethodDef {
                name,
                access_flags,
                method_descriptor,
                instructions,
            })
        } else {
            DiagnosticManager::report_diagnostic(
                &self.lexer.source_file,
                start_span.merge(&self.curr_span()),
                format!("missing method name in method definition"),
            );
            self.errored |= true;

            None
        }
    }

    fn parse_method_defs(&mut self) -> Option<Vec<PhoronMethodDef>> {
        let mut method_defs = Vec::new();

        while let TokenKind::TMethod = self.see().kind {
            method_defs.push(self.parse_method_def()?);
        }

        Some(method_defs)
    }

    /// Body <- FieldDef* MethodDef*
    fn parse_body(&mut self) -> Option<PhoronBody> {
        let field_defs = self.parse_field_defs().or(Some(vec![]))?;
        let method_defs = self.parse_method_defs().or(Some(vec![]))?;

        Some(PhoronBody {
            field_defs,
            method_defs,
        })
    }

    /// SourceFileDef <- SOURCE_keyword FileName newline
    fn parse_sourcefile_def(&mut self) -> Option<PhoronSourceFileDef> {
        let start_span = self.curr_span();

        if let Token {
            kind: TokenKind::TIdent(source_file_str),
            span: _,
        } = self.see()
        {
            let source_file = source_file_str.to_string();
            self.advance();

            Some(PhoronSourceFileDef { source_file })
        } else {
            DiagnosticManager::report_diagnostic(
                &self.lexer.source_file,
                start_span.merge(&self.curr_span()),
                format!("missing source file name for source file definition"),
            );
            self.errored |= true;

            None
        }
    }

    /// Header <- SourceFileDef? (ClassDef / InterfaceDef) SuperDef
    fn parse_header(&mut self) -> Option<PhoronHeader> {
        self.advance();

        Some(match &self.see().kind {
            TokenKind::TSource => {
                self.advance();

                let sourcefile_def = self
                    .parse_sourcefile_def()
                    .or(Some(PhoronSourceFileDef::default()))?;

                let class_or_interface_def = match &self.see().kind {
                    TokenKind::TClass => PhoronClassOrInterface::Class(self.parse_class_def()?),
                    TokenKind::TInterface => {
                        PhoronClassOrInterface::Interface(self.parse_interface_def()?)
                    }
                    tok_kind => {
                        DiagnosticManager::report_diagnostic(
                            &self.lexer.source_file,
                            self.curr_span(),
                            format!(" found {tok_kind:?}, but I expected `.class` or `.interface`"),
                        );
                        self.errored |= true;

                        PhoronClassOrInterface::default()
                    }
                };

                let super_def = self.parse_super_def().or(Some(PhoronSuperDef::default()))?;
                let implements_defs = self.parse_implements_defs().or(Some(vec![]))?;

                PhoronHeader {
                    sourcefile_def,
                    class_or_interface_def,
                    super_def,
                    implements_defs,
                }
            }

            TokenKind::TClass => {
                let sourcefile_def = PhoronSourceFileDef {
                    source_file: self.lexer.src_file().to_string(),
                };

                let class_or_interface_def = PhoronClassOrInterface::Class(
                    self.parse_class_def().or(Some(PhoronClassDef::default()))?,
                );
                let super_def = self.parse_super_def()?;
                let implements_defs = self.parse_implements_defs().or(Some(vec![]))?;

                PhoronHeader {
                    sourcefile_def,
                    class_or_interface_def,
                    super_def,
                    implements_defs,
                }
            }

            TokenKind::TInterface => {
                let sourcefile_def = PhoronSourceFileDef {
                    source_file: self.lexer.src_file().to_string(),
                };

                let class_or_interface_def = PhoronClassOrInterface::Interface(
                    self.parse_interface_def()
                        .or(Some(PhoronInterfaceDef::default()))?,
                );
                let super_def = self.parse_super_def()?;
                let implements_defs = self.parse_implements_defs().or(Some(vec![]))?;

                PhoronHeader {
                    sourcefile_def,
                    class_or_interface_def,
                    super_def,
                    implements_defs,
                }
            }

            tok => {
                DiagnosticManager::report_diagnostic(
                    &self.lexer.source_file,
                    self.curr_span(),
                    format!("{tok:?} cannot start a Phoron header"),
                );

                self.errored |= true;

                PhoronHeader::default()
            }
        })
    }

    /// PhoronProgram <- line_comment* Header Body eof
    pub fn parse(&mut self) -> Option<PhoronProgram> {
        let header = self.parse_header()?;
        let body = self.parse_body()?;

        Some(PhoronProgram { header, body })
    }
}
