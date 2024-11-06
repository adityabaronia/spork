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


    async def buildHTTP(self, agent_build_path, c2):
        baseConfigFile = open("{}/Agent.Profiles.Http/Base.txt".format(agent_build_path.name), "r").read()
        baseConfigFile = baseConfigFile.replace("%UUID%", self.uuid)
        for key, val in c2.get_parameters_dict().items():
            if key == "headers":
                customHeaders = ""
                for item in val:
                    if item == "Host":
                        #baseConfigFile = baseConfigFile.replace("%HOSTHEADER%", val[item])
                        
                        os.system(f"echo '{val[item]}' >> callback-host.txt")
                    elif item == "User-Agent":
                        #baseConfigFile = baseConfigFile.replace("%USERAGENT%", val[item])
                        
                        os.system(f"echo '{val[item]}'  >> user-agent.txt")
                    else:
                        #customHeaders += "this._client.DefaultRequestHeaders.Add(\"{}\", \"{}\");".format(str(item), str(val[item])) + '\n'  
                        os.system(f"echo '{val[item]}'  >> custom-header.txt")
                
                #baseConfigFile = baseConfigFile.replace("%HOSTHEADER%", "")
                #baseConfigFile = baseConfigFile.replace("//%CUSTOMHEADERS%", customHeaders)   
            #elif key == "encrypted_exchange_check":
            #    if val == "T":
            #        baseConfigFile = baseConfigFile.replace(key, "True")
            #    else:
            #        baseConfigFile = baseConfigFile.replace(key, "False")  
            #else:
            #    baseConfigFile = baseConfigFile.replace(str(key), str(val)) 
        #with open("{}/Agent.Profiles.Http/HttpProfile.cs".format(agent_build_path.name), "w") as f:
        #    f.write(baseConfigFile)
        #self.addProfile(agent_build_path, "Http")


    
    # This function is called to build a new payload
    async def build(self) -> BuildResponse:
        # Setup a new build response object
        resp = BuildResponse(status=BuildStatus.Success)


        for c2 in self.c2info:
            profile = c2.get_c2profile()
            if profile["name"] == "http":
                # roots_replace += "<assembly fullname=\"Agent.Profiles.HTTP\"/>" + '\n'
                await self.buildHTTP(agent_build_path, c2)
            elif profile["name"] == "smb":
                #roots_replace += "<assembly fullname=\"Agent.Profiles.SMB\"/>" + '\n'
                #await self.buildSMB(agent_build_path, c2)
            elif profile["name"] == "websocket":
                #roots_replace += "<assembly fullname=\"Agent.Profiles.Websocket\"/>" + '\n'
                #await self.buildWebsocket(agent_build_path, c2)
            elif profile["name"] == "slack":
                #roots_replace += "<assembly fullname=\"Agent.Profiles.Slack\"/>" + '\n'
                #await self.buildSlack(agent_build_path, c2)
            elif profile["name"] == "discord":
                #roots_replace += "<assembly fullname=\"Agent.Profiles.Discord\"/>" + '\n'
                #await self.buildDiscord(agent_build_path, c2)
            else:
                raise Exception("Unsupported C2 profile type for spork: {}".format(profile["name"]))

         return resp
        
        """PayloadUUID = self.uuid
        os.chdir(self.agent_code_path)"""

        # finding all self attributes
        """with open("all_self_attributes.txt", "w") as file:
            for attr in dir(self):
                # Filter out dunder (special) methods and properties
                if not attr.startswith("__"):
                    # Get the value of each attribute
                    value = getattr(self, attr)
                    file.write(f"{attr}: {value}\n")

        
        # finding UUID
        os.system(f"echo '{PayloadUUID}' >> testingbuild.txt")
        
        # finding c2info
        if len(self.c2info) != 1:
            resp.set_status(BuildStatus.Error)
        profile = self.c2info[0]
        parameters_dict = profile.get_parameters_dict()
        for key, value in parameters_dict.items():
            os.system(f"echo '{key}' : '{value}' >> c2info.txt")"""

       
        
        
