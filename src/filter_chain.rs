use std::boxed::Box;

use notice::Notice;
use config::Config;

pub struct FilterChain {
    filters: Vec<Box<Fn(&mut Notice)>>,
}

impl FilterChain {
    pub fn new(config: &Config) -> FilterChain {
        let mut filter_chain = FilterChain {
            filters: Vec::new(),
        };

        let root_directory = config.root_directory.clone();

        if !config.root_directory.is_empty() {
            filter_chain.add_filter(move |notice| {
                for error in &notice.errors {
                    for frame in &error.backtrace {
                        if frame.starts_with(&root_directory) {
                            println!("WOLOWOWOOWOWOWOWOWO");
                        }
                    }
                }
            });
        }

        filter_chain
    }

    pub fn add_filter<F>(&mut self, filter: F)
        where F: Fn(&mut Notice) + 'static
    {
        self.filters.push(Box::new(filter));
    }

    pub fn refine(&self, notice: &mut Notice) {
        for filter in &self.filters {
            filter(notice);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Error;
    use super::FilterChain;
    use config::Config;
    use notice::Notice;

    #[test]
    fn refine_runs_a_callback_for_given_notice() {
        let config = Config::new();
        let mut filter_chain = FilterChain::new(&config);
        let mut notice = Notice::new(&config, Error::last_os_error());

        assert_eq!("Error", notice.errors[0].type_);
        filter_chain.add_filter(|notice| {
            notice.errors[0].type_ = "Bingo".to_owned();
        });
        assert_eq!("Error", notice.errors[0].type_);

        filter_chain.refine(&mut notice);
        assert_eq!("Bingo", notice.errors[0].type_);
    }

    #[test]
    fn refine_filters_out_root_directory_if_it_is_specified() {
        panic!("fail");
    }
}
