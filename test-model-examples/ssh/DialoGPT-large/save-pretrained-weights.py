import os, sys
from transformers import AutoModelForCausalLM, AutoTokenizer

try:
    # Define the subfolder name to save the weights
    subfolder_name = "weights/pretrained"

    # Check if the subfolder exists if not create it
    save_folder_path = os.path.join(os.getcwd(), subfolder_name)
    os.makedirs(save_folder_path, exist_ok=True)

    # Load pre-trained model and tokenizer
    model = AutoModelForCausalLM.from_pretrained("microsoft/DialoGPT-large")
    tokenizer = AutoTokenizer.from_pretrained("microsoft/DialoGPT-large")

    # Save the model weights to the specified subfolder
    model.save_pretrained(save_folder_path)
    tokenizer.save_pretrained(save_folder_path)

    sys.exit(0)

except:
    sys.exit(1)
