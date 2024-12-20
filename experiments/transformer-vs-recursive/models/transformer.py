import torch
import torch.nn as nn
import math
import torch.nn.functional as F
from transformers import BertModel, BertConfig

class PositionEncoding(nn.Module):
    def __init__(self, d_model, max_seq_len):
        super(PositionEncoding, self).__init__()
        position_encoding = self.create_sinusoidal_encoding(max_seq_len, d_model)
        self.register_buffer("position_encoding", position_encoding)

    def create_sinusoidal_encoding(self, max_seq_len, d_model):
        position = torch.arange(max_seq_len).unsqueeze(1)
        div_term = torch.exp(
            torch.arange(0, d_model, 2) * (-math.log(10000.0) / d_model)
        )
        pe = torch.zeros(1, max_seq_len, d_model)
        pe[0, :, 0::2] = torch.sin(position * div_term)
        pe[0, :, 1::2] = torch.cos(position * div_term)
        return pe

    def forward(self, x):
        return x + self.position_encoding[:, : x.size(1), :]  # pyright: ignore

class EncoderOnlyTransformer(nn.Module):
    def __init__(
        self, vocab_size, output_dim, num_layers, num_heads, d_model, max_seq_len
    ):
        super(EncoderOnlyTransformer, self).__init__()
        self.vocab_size = vocab_size
        self.embedding = nn.Embedding(vocab_size, d_model)
        self.position_encoding = PositionEncoding(d_model, max_seq_len)
        encoder_layer = nn.TransformerEncoderLayer(d_model=d_model, nhead=num_heads)
        self.transformer_encoder = nn.TransformerEncoder(
            encoder_layer, num_layers=num_layers
        )
        self.fc_out = nn.Linear(d_model, output_dim)

    def forward(self, x):
        # Check if input indices are within the valid range
        if torch.any(x >= self.vocab_size) or torch.any(x < 0):
            invalid_indices = torch.where((x >= self.vocab_size) | (x < 0))
            raise ValueError(
                f"Input contains indices outside the valid range [0, {self.vocab_size - 1}]. "
                f"Invalid indices: {invalid_indices}. "
                f"Values at these indices: {x[invalid_indices]}"
            )

        # Convert word indices to embeddings
        x = self.embedding(x)
        x = self.position_encoding(x)
        x = self.transformer_encoder(x)
        x = self.fc_out(x)
        return x

class CustomBERTModel(nn.Module):
    def __init__(self, vocab_size, output_dim, num_layers, num_heads, hidden_dim, max_seq_len, **kwargs):
        super(CustomBERTModel, self).__init__()
        config = BertConfig(
            vocab_size=vocab_size,
            hidden_size=hidden_dim,
            num_hidden_layers=num_layers,
            num_attention_heads=num_heads,
            intermediate_size=2 * hidden_dim, # ?
            hidden_dropout_prob=0.1,
            attention_probs_dropout_prob=0.1,
            max_position_embeddings=max_seq_len,
        )
        self.bert = BertModel(config)
        self.regression = nn.Linear(hidden_dim, output_dim)

    def forward(self, x):
        outputs = self.bert(x)
        sequence_output = outputs.last_hidden_state
        logits = self.regression(sequence_output)
        return logits


device = "cuda:0"
eotf = EncoderOnlyTransformer(10, 10, 3, 2, 10, 100).to(device)
