import asyncio
import json
import os
import pathlib
import sys
import tempfile
import traceback
from shutil import copytree
from mythic_container.PayloadBuilder import *

# pylint: disable=too-many-locals,too-many-branches,too-many-statements


# Class defining information about the Thanatos payload
class spork(PayloadType):
    name = "spork"  # Name of the payload
    file_extension = "exe"  # default file extension to use when creating payloads
    author = "@smoke"  # authors

    # Platforms that spork supports
    supported_os = [
        SupportedOS.Windows
    ]
    wrapper = False
    wrapped_payloads = []
    # Description of the payload in Mythic
    description = "Windows agent written in Rust"

    # Payload does not support dynamic loading
    supports_dynamic_loading = False
    mythic_encrypts = True
    
    # Supported C2 profiles for spork
    c2_profiles = ["http"]
    build_parameters = [
        BuildParameter(
            name="output_type",
            parameter_type=BuildParameterType.ChooseOne,
            choices=["Executable", "Shellcode", "Service", "dll"],
            default_value="Executable",
            description="Output as shellcode, executable, service or a dll.",
        )
    ]


    build_steps = [
        BuildStep(step_name="Gathering selected commands", step_description="Making sure all commands required are there for use"),
        BuildStep(step_name="Configuring", step_description="Stamping in configuration values(UUID)"),
        BuildStep(step_name="Building", step_description="building the agent after all the configurations")
    ]
    
    agent_path = pathlib.Path(".") / "spork" / "mythic"
    agent_code_path = pathlib.Path(".") / "spork" / "agent_code"
    agent_icon_path = agent_path / "agent_icon" / "spork.svg"

    # This function is called to build a new payload
    async def build(self) -> BuildResponse:
        # Setup a new build response object
        resp = BuildResponse(status=BuildStatus.Success)
        PayloadUUID = self.uuid
        os.chdir(self.agent_code_path)
        # finding UUID
        os.system(f"echo '{PayloadUUID}' >> testingbuild.txt")
        # finding callbackhost
        if len(self.c2info) != 1:
            resp.set_status(BuildStatus.Error)
        profile = self.c2info[0]
        callback = profile.get_parameters_dict()['callback_host']
        os.system(f"echo '{callback}' >> testingc2info")
        return resp
