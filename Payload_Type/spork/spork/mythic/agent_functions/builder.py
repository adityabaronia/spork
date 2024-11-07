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
        resp = BuildResponse(status=BuildStatus.Error)
        baseConfigFile = open("{}/src/network-configuration.txt".format(self.agent_code_path), "r").read()
        baseConfigFile = baseConfigFile.replace("%UUID%", self.uuid)
        for c2 in self.c2info:
            profile = c2.get_c2profile()
            if profile["name"] == "http":   
                os.system(f"echo 'http hai' >> http.txt")
                for key, val in c2.get_parameters_dict().items():
                    os.system(f"echo '{key}': '{val}' >> c2infodata.txt")
                    if key == "headers":
                        customHeaders = ""
                        for item in val:
                            if item == "Host":
                                baseConfigFile = baseConfigFile.replace("%USER-AGENT%", val[item])
                            elif item == "User-Agent":
                                baseConfigFile = baseConfigFile.replace("%USER-AGENT%", val[item])
                            else:
                                customHeaders += "this._client.DefaultRequestHeaders.Add(\"{}\", \"{}\");".format(str(item), str(val[item])) + '\n'
                                baseConfigFile = baseConfigFile.replace("%USERAGENT%", customHeaders)
                    if key == "callback_host":
                        #baseConfigFile = baseConfigFile.replace("%CALLBACK-HOST%", self.callback_host)
                        
                        value = val[8:]
                        os.system(f"echo 'value' > host-server.txt")
                        baseConfigFile = baseConfigFile.replace("%CALLBACK-HOST%", value)
        resp = BuildResponse(status=BuildStatus.Success)
        return resp
       

       
        
        
