use {don_error::*, std::process::Command};

const FF_SYNC_COMMAND: &str = "ffsclient";
const TRUNCATE_STDERR_OUTPUT_TO_CHARS: usize = 2000;

#[derive(Debug, serde::Deserialize)]
pub struct Client {
    pub username: String,
    pub password: String,
}

impl Client {
    pub(crate) fn login(&self) -> DonResult<()> {
        self.try_command(&["login", &self.username, &self.password])
    }

    fn ffsclient_command(&self, args: &[&str]) -> Command {
        let mut command = Command::new(&FF_SYNC_COMMAND);
        args.iter().for_each(|arg| {
            command.arg(arg);
        });
        command
    }

    pub(crate) fn try_command_and_then<D>(
        &self,
        args: &[&str],
        on_successful_output: impl Fn(String) -> DonResult<D>,
    ) -> DonResult<D> {
        self.try_command_and_then_inner(args, on_successful_output, false)
    }

    pub(crate) fn try_command_and_then_inner<D>(
        &self,
        args: &[&str],
        on_successful_output: impl Fn(String) -> DonResult<D>,
        has_already_tried_to_login: bool,
    ) -> DonResult<D> {
        let mut command = self.ffsclient_command(args);
        let output = command.output()?;
        if output.status.success() {
            on_successful_output(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            // We could read the stderr and check some specific messages to only try to login in
            // some specific cases but I fear I might be missing some scenarios + trying
            // to login doesn't cost that much
            if !has_already_tried_to_login {
                self.login()?;
                self.try_command_and_then_inner(args, on_successful_output, true)
            } else {
                Err(err_msg!("{}", {
                    let mut stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    stderr.truncate(TRUNCATE_STDERR_OUTPUT_TO_CHARS);
                    stderr
                }))
            }
        }
    }

    pub(crate) fn try_command(&self, args: &[&str]) -> DonResult<()> {
        self.try_command_and_then(args, |_| Ok(()))
    }
}
