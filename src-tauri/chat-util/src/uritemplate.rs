use uritemplate::{IntoTemplateVar, TemplateVar};

/// A URI Template
///
/// See IETF RFC 6570
pub struct UriTemplate(uritemplate::UriTemplate);

pub enum IfEmpty {
    /// Assign empty value
    Set,
    /// Assign comma (`,`)
    Comma,
    /// Omit the whole variable
    Skip,
}

impl UriTemplate {
    /// Prepare a new template for evaluation
    /// Takes a template string and returns a new [`UriTemplate`]
    #[must_use]
    pub fn new(template: &str) -> Self {
        Self(uritemplate::UriTemplate::new(template))
    }

    /// Bind a variable to a scalar value.
    #[allow(clippy::needless_pass_by_value)]
    pub fn set_scalar(&mut self, varname: impl AsRef<str>, var: impl ToString) -> &mut Self {
        self.0
            .set(varname.as_ref(), TemplateVar::Scalar(var.to_string()));
        self
    }

    /// Bind a variable to a scalar value, if there is a value.
    /// If the value is `None`, omit the variable
    pub fn set_optional_scalar(
        &mut self,
        varname: impl AsRef<str>,
        var: Option<impl ToString>,
    ) -> &mut Self {
        match var {
            Some(var) => self.set_scalar(varname, var),
            None => self,
        }
    }

    /// Bind a variable to a list value.
    pub fn set_list<T: ToString>(
        &mut self,
        varname: impl AsRef<str>,
        var: impl IntoIterator<Item = T>,
    ) -> &mut Self {
        self.0.set(
            varname.as_ref(),
            TemplateVar::List(var.into_iter().map(|val| val.to_string()).collect()),
        );
        self
    }

    /// Bind a variable to a list value, specifying what happens if the value us empty.
    pub fn set_list_with_if_empty<T: ToString>(
        &mut self,
        varname: impl AsRef<str>,
        var: impl IntoIterator<Item = T>,
        if_empty: IfEmpty,
    ) -> &mut Self {
        let mut var = var.into_iter();
        match var.next() {
            Some(first) => self.set_list(varname, std::iter::once(first).chain(var)),
            None => match if_empty {
                IfEmpty::Set => self.set_list(varname, std::iter::empty::<String>()),
                IfEmpty::Comma => self.set_scalar(varname, ","),
                IfEmpty::Skip => self,
            },
        }
    }

    /// Bind a variable to an associative array value.
    pub fn set_assoc(
        &mut self,
        varname: impl AsRef<str>,
        var: impl IntoIterator<Item = (String, String)>,
    ) -> &mut Self {
        self.0.set(
            varname.as_ref(),
            TemplateVar::AssociativeArray(var.into_iter().collect()),
        );
        self
    }

    /// Bind the variable to a raw [`IntoTemplateVar`] implementor.
    ///
    /// This is a lower-level API, suitable for utilizing [`uritemplate`]
    /// crate API
    pub fn set_template_var(
        &mut self,
        varname: impl AsRef<str>,
        var: impl IntoTemplateVar,
    ) -> &mut Self {
        self.0.set(varname.as_ref(), var);
        self
    }

    /// Apply a function to the value and return self.
    ///
    /// Useful for maintaining method chains
    pub fn tap(&mut self, f: impl FnOnce(&mut Self)) -> &mut Self {
        f(self);
        self
    }

    /// Delete a variable binding set before
    pub fn delete(&mut self, varname: impl AsRef<str>) -> bool {
        self.0.delete(varname.as_ref())
    }

    /// Build a URL from the template and bound variable values.
    pub fn build(&mut self) -> String {
        self.0.build()
    }
}

impl std::fmt::Debug for UriTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UriTemplate")
    }
}
