import sys, argparse
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer

#############################################################################################
# Testing STDERR and STDOUT
# class ColoredStream:
#     def __init__(self, stream, color):
#         self.stream = stream
#         self.color = color

#     def write(self, text):
#         self.stream.write(f"{self.color}{text}{Color.RESET}")

# # ANSI escape codes for text colors
# class Color:
#     RED = '\033[91m'
#     GREEN = '\033[92m'
#     YELLOW = '\033[93m'
#     RESET = '\033[0m'

# # Override sys.stdout and sys.stderr
# sys.stdout = ColoredStream(sys.stdout, Color.GREEN)
# sys.stderr = ColoredStream(sys.stderr, Color.RED)
#############################################################################################


# Define tokens for controlling the program
READY_TOKEN = "@!#READY#!@"
EXIT_TOKEN = "@!#EXIT#!@"
START_TOKEN = "@!#START#!@"
STOP_TOKEN = "@!#STOP#!@"


# Define max length of the context
MAX_LENGTH = 1000


#############################################################################################
# Fetch which weights to load from the command line arguments
parser = argparse.ArgumentParser(description='Run inference on a pre-trained DialoGPT model.')
parser.add_argument('--weights', type=str, default="pretrained", help='The subfolder name to load the weights from.')
args = parser.parse_args()

#############################################################################################
# Load model from the specified subfolder
try:
    # Define the subfolder name to load the weights
    subfolder_name = "weights/" + args.weights

    # Load pre-trained model and tokenizer
    model = AutoModelForCausalLM.from_pretrained(subfolder_name)
    tokenizer = AutoTokenizer.from_pretrained(subfolder_name)

except:
    print("Failed to load the model from the specified subfolder {}.".format(subfolder_name), file=sys.stderr)
    sys.exit(1)

# Set the model to evaluation mode
model.eval()


#############################################################################################
# Initialize the chat history
chat_history_ids = None
step = 0

# Print the running token to indicate that the model is ready
print(READY_TOKEN)

# Start the REPL loop
while True:
    # Read line from stdin
    line = input()

    # Check if the program should exit
    if line == EXIT_TOKEN:
        sys.exit(0)

    # Check if the program should start
    if line == START_TOKEN:
        input_string = input()

        # Check the length of the input string
        if len(input_string) == 0:
            print("EMPTY", file=sys.stderr)
            continue
        elif len(input_string) > MAX_LENGTH:
            print("TOO LONG".format(MAX_LENGTH), file=sys.stderr)
            continue
        else:
            line = input()

            # Check if the context stops
            if line != STOP_TOKEN:
                print("Invalid stop token. Please enter a valid stop token.", file=sys.stderr)
                continue
            else:
                # Encode the input string
                new_user_input_ids = tokenizer.encode(input_string + tokenizer.eos_token, return_tensors='pt')

                # Append the new user input tokens to the chat history
                bot_input_ids = torch.cat([chat_history_ids, new_user_input_ids], dim=-1) if step > 0 else new_user_input_ids

                # Generate a response
                chat_history_ids = model.generate(bot_input_ids, max_length=1000, pad_token_id=tokenizer.eos_token_id)

                # Decode and print the response
                print(START_TOKEN)
                print(tokenizer.decode(chat_history_ids[:, bot_input_ids.shape[-1]:][0], skip_special_tokens=True))
                print(STOP_TOKEN)

                # Increment the step
                step = step + 1