
pub struct ContextBuilder {
    pub operating_system: Option<String>,
    pub hostname: Option<String>,
    pub language: Option<String>,
    pub environment: Option<String>,
    pub severity: Option<String>,
    pub version: Option<String>,
    pub url: Option<String>,
    pub root_directory: Option<String>,
    pub user: Option<ContextUser>,
    pub route: Option<String>,
    pub http_method: Option<String>,
}

impl ContextBuilder {
    pub fn new() -> ContextBuilder {
        ContextBuilder {
            operating_system: None,
            hostname: None,
            language: None,
            environment: None,
            severity: None,
            version: None,
            url: None,
            root_directory: None,
            user: None,
            route: None,
            http_method: None
        }
    }

    /// Set the operating system on the ContextBuilder
    pub fn operating_system(mut self, os: String) -> ContextBuilder {
        self.operating_system = Some(os);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn hostname(mut self, hostname: String) -> ContextBuilder {
        self.hostname = Some(hostname);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn language(mut self, language: String) -> ContextBuilder {
        self.language = Some(language);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn environment(mut self, environment: String) -> ContextBuilder {
        self.environment = Some(environment);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn severity(mut self, severity: String) -> ContextBuilder {
        self.severity = Some(severity);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn version(mut self, version: String) -> ContextBuilder {
        self.version = Some(version);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn url(mut self, url: String) -> ContextBuilder {
        self.url = Some(url);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn root_directory(mut self, root_directory: String) -> ContextBuilder {
        self.root_directory = Some(root_directory);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn user(mut self, user: ContextUser) -> ContextBuilder {
        self.user = Some(user);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn route(mut self, route: String) -> ContextBuilder {
        self.route = Some(route);
        self
    }

    /// Set the operating system on the ContextBuilder
    pub fn http_method(mut self, http_method: String) -> ContextBuilder {
        self.http_method = Some(http_method);
        self
    }

    pub fn build(self) -> Context {
        Context {
            notifier: &CONTEXT_NOTIFIER,
            operating_system: self.operating_system,
            hostname: self.hostname,
            language: self.language,
            environment: self.environment,
            severity: self.severity,
            version: self.version,
            url: self.url,
            root_directory: self.root_directory,
            user: self.user,
            route: self.route,
            http_method: self.http_method
        }
    }

}

#[derive(Debug, Serialize, Clone)]
pub struct Context {
    // Builtin notifier
    pub notifier: &'static ContextNotifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename="os")]
    pub operating_system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename="rootDirectory")]
    pub root_directory: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ContextUser>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,

    #[serde(rename="httpMethod")]
    pub http_method: Option<String>
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::new()
    }
}

/// This type is not intended to be used beyond the const CONTEXT_NOTIFIER
#[derive(Debug, Serialize, Clone)]
pub struct ContextNotifier {
    name: &'static str,
    version: &'static str,
    url: &'static str,
}

pub const CONTEXT_NOTIFIER: ContextNotifier = ContextNotifier {
    name: crate::NOTIFIER_NAME,
    version: crate::NOTIFIER_VERSION,
    url: crate::NOTIFIER_URL
};

#[derive(Debug, Serialize, Clone)]
pub struct ContextUser {
    id: Option<String>,
    name: Option<String>,
    email: Option<String>
}


