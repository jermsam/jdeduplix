# JDeduplix: AI-Powered Smart Deduplication System

JDeduplix is a cutting-edge deduplication system that leverages artificial intelligence to provide intelligent, accurate, and efficient data deduplication across multiple data types.

## 📊 System Architecture

```mermaid
graph TD;

    %% USER INTERACTION
    subgraph "User Interaction"
        A[User Uploads Data] -->|AI-Powered Detection| B[Smart Classifier]
    end

    %% BACKEND
    subgraph "Backend (Rust + AI)"
        B -->|Splits & Routes Data| C[Deduplication Engine]
        
        %% Deduplication Strategies
        C -->|Processes Data| D[AI-Enhanced Deduplication Strategies]
        D -->|Exact & AI-Fuzzy Matching| D1[ML Text Deduper] & D2[GNN JSON Deduper] & D3[CNN Image Deduper] & D4[Deep Learning Binary Deduper] 
        
        %% ML-Assisted Conflict Resolution
        D -->|Sends Predictions| R[ML Conflict Resolver]
        R -->|Human-in-the-Loop Review| S[User Feedback]
        R -->|Confidence-Weighted Decisions| T[Threshold Auto-Tuning]

        %% AI Indexing & Storage Layer
        C -->|Indexes & Stores| E[Smart Cache & Vector Indexing]
        E -->|Retrieves Data| G1[FAISS HNSW Vector Search] & G2[Perceptual Hash] & G3[Graph Matching]

    end

    %% STORAGE OPTIONS
    subgraph "Storage (Optional)"
        E -->|Pluggable Options| F1[Sled DB] & F2[File System] & F3[Vector DB - Weaviate or Pinecone]
    end

    %% UI
    subgraph "UI (Tauri + Vue)"
        G[User Views Results] -->|AI-Suggested Review| H[Manual Conflict Resolution]
        G -->|Visualizes Similarity & Explanation| I[AI Confidence Score & Heatmap]
        G -->|Exports Data| J[Download or Export]
    end

    %% FINAL OUTPUT
    C -->|Sends AI-Enhanced Results| G
```

## 🌟 Key Features

- **Smart Classification**: AI-powered detection and routing of different data types
- **Multi-Modal Deduplication**: Specialized engines for text, JSON, images, and binary data
- **ML-Enhanced Matching**: Combines exact and fuzzy matching using advanced AI models
- **Intelligent Conflict Resolution**: Machine learning-assisted conflict handling with human oversight
- **High-Performance Search**: Vector-based similarity search using FAISS HNSW
- **Flexible Storage**: Pluggable storage backends supporting Sled DB, file system, and vector databases

## 🏗️ Architecture

### Frontend (Tauri + Vue)
- Modern, responsive UI for data visualization and management
- AI confidence score displays and similarity heatmaps
- Interactive conflict resolution interface
- Export and download capabilities

### Backend (Rust + AI)
- **Deduplication Engine**:
  - ML Text Deduper
  - GNN JSON Deduper
  - CNN Image Deduper
  - Deep Learning Binary Deduper
  
- **Smart Indexing**:
  - FAISS HNSW Vector Search
  - Perceptual Hashing
  - Graph Matching Algorithms

- **ML Conflict Resolution**:
  - Confidence-weighted decision making
  - Automated threshold tuning
  - Human-in-the-loop review system

### Storage Options
- Sled DB for high-performance local storage
- File system integration for simple deployments
- Vector DB support (Weaviate/Pinecone) for advanced similarity search

## 🚀 Getting Started

[Coming Soon]

## 🛠️ Development

[Coming Soon]

## 📚 Documentation

[Coming Soon]

## 🤝 Contributing

[Coming Soon]

## 📄 License

[Coming Soon]

## 🔗 Links

[Coming Soon]