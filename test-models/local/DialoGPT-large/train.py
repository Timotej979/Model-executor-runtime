import torch
from transformers import GPT2LMHeadModel, GPT2Tokenizer, GPT2Config
from transformers import TextDataset, DataCollatorForLanguageModeling
from transformers import Trainer, TrainingArguments


#############################################################################################
# Fetch the training dataset from the command line arguments
parser = argparse.ArgumentParser(description='Train a DialoGPT model.')
# Dataset and weights
parser.add_argument('--dataset', type=str, default="train.txt", help='The file name of the training dataset in subfolder.')
parser.add_argument('--oldWeights', type=str, default="pretrained", help='The subfolder name to load the weights from.')
parser.add_argument('--newWeights', type=str, default="old", help='The subfolder name to load the weights from.')
# Training hyperparameters
parser.add_argument('--block_size', type=int, default=128, help='The block size.')
parser.add_argument('--mlm', type=bool, default=False, help='Set to True if you have masked language modeling objective.')
parser.add_argument('--epochs', type=int, default=3, help='The number of training epochs.')
parser.add_argument('--batch_size', type=int, default=4, help='The batch size.')
parser.add_argument('--save_steps', type=int, default=10000, help='The number of steps to save the model.')
parser.add_argument('--save_total_limit', type=int, default=2, help='The number of checkpoints to save.')
args = parser.parse_args()
#############################################################################################

#############################################################################################
# Define the model configuration
model_name = "microsoft/DialoGPT-large"
model_config = GPT2Config.from_pretrained(model_name)

# Load pre-trained DialoGPT model and tokenizer
model = GPT2LMHeadModel.from_pretrained(model_name, config=model_config)
tokenizer = GPT2Tokenizer.from_pretrained(model_name)

# Load your training dataset (make sure it's in text format)
train_dataset = TextDataset(
    tokenizer=tokenizer,
    file_path='datasets/' + args.dataset,
    block_size=args.block_size
)

# Create data collator for language modeling
data_collator = DataCollatorForLanguageModeling(
    tokenizer=tokenizer,
    mlm=args.mlm,
)

# Configure training arguments
training_args = TrainingArguments(
    output_dir="./training_output",
    overwrite_output_dir=True,
    num_train_epochs=args.epochs,
    per_device_train_batch_size=args.batch_size,
    save_steps=args.save_steps,
    save_total_limit=args.save_total_limit,
)

# Initialize Trainer
trainer = Trainer(
    model=model,
    args=training_args,
    data_collator=data_collator,
    train_dataset=train_dataset,
)

# Train the model
trainer.train()

# Save the trained model
model.save_pretrained(args.newWeights)
tokenizer.save_pretrained(args.newWeights)