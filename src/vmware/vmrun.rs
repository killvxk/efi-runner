#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmState {
    Running,
    Stopped,
}

impl super::VMWare {
    pub fn vmrun(&self, args: &Vec<&str>) -> anything::Result<String> {
        let output = std::process::Command::new("vmrun").args(args).output()?;
        let stdout = String::from_utf8(output.stdout)?;

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(stdout.into())
        }
    }

    pub fn state(&self) -> anything::Result<VmState> {
        let list = self.vmrun(&vec!["list"])?;

        if list.contains(self.vmx_path.as_str()) {
            Ok(VmState::Running)
        } else {
            Ok(VmState::Stopped)
        }
    }

    pub fn start(&self, nogui: bool) -> anything::Result<()> {
        let mut args = vec!["start", self.vmx_path.as_str()];
        if nogui {
            args.push("nogui");
        }

        self.vmrun(&args)?;
        Ok(())
    }

    pub fn stop(&self, hard: bool) -> anything::Result<()> {
        let mut args = vec!["stop", self.vmx_path.as_str()];

        if hard {
            args.push("hard");
        } else {
            args.push("soft");
        }

        self.vmrun(&args)?;
        Ok(())
    }
}
