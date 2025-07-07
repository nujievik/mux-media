use crate::{
    GlobSetPattern, IsDefault, ToJsonArgs, from_arg_matches, json_arg, push_true_json_args,
};
use clap::{ArgMatches, Error, FromArgMatches};

#[derive(Clone)]
pub struct Retiming {
    pub rm_segments: Option<GlobSetPattern>,
    pub no_linked: bool,
    pub less: bool,
}

impl IsDefault for Retiming {
    fn is_default(&self) -> bool {
        self.rm_segments.is_none() && !self.no_linked && !self.less
    }
}

impl FromArgMatches for Retiming {
    from_arg_matches!(@fn);
    from_arg_matches!(@fn_update);

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        Ok(Self {
            rm_segments: from_arg_matches!(matches, GlobSetPattern, RmSegments, @no_default),
            no_linked: from_arg_matches!(matches, bool, NoLinked, || false),
            less: from_arg_matches!(matches, bool, LessRetiming, || false),
        })
    }

    fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error> {
        from_arg_matches!(@upd, self, matches, @opt_field; rm_segments, GlobSetPattern, RmSegments);

        from_arg_matches!(
            @upd, self, matches;
            no_linked, bool, NoLinked,
            less, bool, LessRetiming
        );

        Ok(())
    }
}

impl ToJsonArgs for Retiming {
    fn to_json_args(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        if let Some(pat) = &self.rm_segments {
            if !pat.raw.is_empty() {
                args.push(json_arg!(RmSegments));
                args.push(pat.raw.to_string());
            }
        }

        push_true_json_args!(args, self; no_linked, NoLinked, less, LessRetiming);

        args
    }
}
