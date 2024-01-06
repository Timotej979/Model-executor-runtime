# Tests for model examples

This folder presents the approach on how the driver should be used for different connection types (Local or SSH) of a typical model pulled from Hugging Faces. For testing we first need to load the pyton/conda environment with the required python libraries and then execute the tests for each of the models in the respected connection type folders.

```bash
python3 -m venv transformer-venv
source transformer-venv/bin/activate
pip install 'transformers[torch]'
```

```bash
conda env create -f environment.yml
conda activate transformer-venv
```

If you want to deactivate and remove the environments do the following:

```bash
deactivate
rm -rf transformer-venv
```

```bash
conda deactivate
conda remove -n transformer-venv --all
```


As expected for testing the SSH connection type we pull the OpenSSH server from linuxservers.io to accurately mimic the behaviour of the production grade model running on a remote server connected via SSH. You can connect to this SSH server using the following example command:

```bash
ssh admin@$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' openssh-server) -p 2222
```