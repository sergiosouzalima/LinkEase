<div id="portuguese"></div>

# LinkEase

<p align="right">(<a href="#english">English version</a>)</p>

Este repositório contém código Rust para o LinkEase, um aplicativo de linha de
comando que processa dados de conexões do LinkedIn.

Esta aplicativo usa como base de dados o arquivo Connections.csv que pode ser
solicitado ao LinkedIn.

Com este arquivo, o aplicativo filtra suas conexões do LinkedIn com base na
profissão e você pode importar essas conexões filtradas para uma planilha do Excel,
por exemplo.

Assim você pode manter contato e melhorar sua comunicação com profissionais
de um perfil desejado.

## Como usar o LinkEase:

### Baixe o LinkEase para seu computador:
Para usar o Linkease você deverá, em primeiro lugar, baixar o
arquivo linkease para sua máquina.

O linkease está disponível neste repositório para os três principais
sistemas operaconais: Linux, Windows e MacOS.

Localize a pasta correspondente ao seu sistema operacional e
baixe o arquivo "linkease" para uma pasta de sua preferência
em seu computador.

### Baixe o arquivo Connections.csv:

O primeiro passo para baixar o arquivo Connections.csv é estar conectado
à sua conta do LinkedIn.

### Como solicitar o arquivo Connections.csv do LinkedIn:

Para fazer isso, siga os passos abaixo:

1. Certifique-se que você está conectado à sua conta no LinkedIn.
2. Clique no menu suspenso 'Eu'.
3. Clique em 'Configurações e Privacidade'.
4. Selecione 'Privacidade de dados'.
5. Clique em 'Obtenha uma cópia dos seus dados'.
6. Clique em 'Conexões'.
7. Clique no botão 'Solicitar arquivo'.
8. Você deve receber sua exportação no endereço de email associado à sua conta
do LinkedIn dentro de 10 minutos.
9. Em seu email, clique em "baixar seu arquivo de dados usando este link"
para baixar o arquivo.
10. O arquivo baixado está compactado. Lembre-se de descompacta-lo.

### Execute o programa LinkEase:

Abra o terminal e navegue até o diretório em que você salvou o
programa LinkEase.

Posicione o arquivo `Connections.csv` no mesmo diretório onde você salvou o LinkEase.

Em seguida, execute o LinkEase: digite `linkease` e pressione ENTER.

### Filtrar suas conexões:

O LinkEase criará um novo arquivo chamado `result_connections.csv` que contém
conexões com base em sua profissão.

Você pode mudar o filtro de profissão no arquivo `config.toml`.

### Importe as conexões filtradas para uma planilha do Excel:

Depois de executar o LinkEase, você pode importar o arquivo `result_connections.csv`
para uma planilha do Excel para análise e organização adicionais.

## Recursos do LinkEase

### Filtrar conexões

O LinkEase é projetado para tornar fácil para você gerenciar e comunicar
com suas conexões do LinkedIn.

Um de seus recursos principais é a capacidade de filtrar suas conexões com
base em sua profissão, tornando-o ideal para Recrutadores de TI ou Recrutadores de Tecnologia.

### Se você é um Recrutador de TI/Tecnologia

Por padrão, o LinkEase está configurado para buscar conexões com cargos que contenham os seguintes termos:

position_filter_for = ["App", "Application", "Back End", "Desenvolvedor", "Developer",
"Engineer", "Front End", "Full Stack", "Mobile", "Programador", "Programmer", "Soft",
"Software", "Sistemas", "Systems", "Web"]

No entanto, você pode facilmente alterar ou aprimorar esses termos editando o arquivo `config.toml`.

### Se você está procurando um emprego

Se você está procurando emprego, pode mudar esses termos editando o arquivo `config.toml`
e adicionando, por exemplo, os seguintes termos:

position_filter_for = ["Attraction", "Atração", "Benefit", "Benefits",
"Benefício", "Benefícios", "Gente", "RH", "Recursos Humanos", "Pessoas",
"R&S", "Recrutador", "Recrutador", "Recrutadora", "Talento", "Talentos",
"Human Resources","Headhunter"]

***

<div id="english"></div>

# LinkEase

<p align="right">(<a href="#portuguese">Versao em Portugues</a>)</p>

This repository contains Rust code for a command-line application that
processes LinkedIn connections data (CSV format file) and enhances
communication between the user and their connections.

With LinkEase, you can easily filter your LinkedIn connections based on
occupation and import the filtered connections into an Excel spreadsheet.

## How to Use LinkEase:

### Download LinkEase to your computer:
To use Linkease you must first download the linkease file to your machine.

Linkease is available in this repository for all three major operating
systems: Linux, Windows and MacOS.

Locate the folder corresponding to your operating system and
download the "linkease" file to a folder of your choice on your computer.

### Download the Connections.csv file:

The first step to download Connections.csv file is to be connected to
your LinkedIn account.

### How to request Connections.csv file from LinkedIn:

To do this, follow the steps below:

1. Make sure you are connected to your LinkedIn account.
2. Click the 'Me' dropdown menu.
3. Click 'Settings & Privacy'.
4. Select 'Data privacy'.
5. Click on 'Get a copy of your data'.
6. Click on 'Connections'.
7. Click on 'Request archive' button.
8. You should receive your export at the email address associated with your
LinkedIn account within 10 minutes.
9. In your email, click "download your data archive using this link.".
10. The downloaded file is zipped. Remember to unzip it.

### Run the LinkEase program:

Open the terminal and navigate to the directory where you have saved the
LinkEase program.

Place the `Connections.csv` file in the same directory where you've just
saved LinkEase.

Then, run LinkEase: simply type `linkease` and hit ENTER.

### Filter your connections:

LinkEase will create a new file named `result_connections.csv` that contains
connections based on their position (role), as specified by the user.

You can change the position (role) filter in the `config.toml` file.

### Import the filtered connections into an Excel spreadsheet:

Once you have run LinkEase, you can import the `result_connections.csv`
file into an Excel spreadsheet for further analysis and organization.

## LinkEase features
### Filter for Connections

LinkEase is designed to make it easy for you to manage and communicate
with your LinkedIn connections.

One of its key features is the ability to filter your connections based
on their occupation, making it ideal for IT Recruiters or Tech Recruiters.


### If you are an IT/Tech Recruiter

By default, LinkEase is configured to search for connections with job titles that contain the following terms:

position_filter_for = ["App", "Application", "Back End", "Desenvolvedor", "Developer","Engineer", "Front End", "Full Stack", "Mobile", "Programador",  "Programmer", "Soft",
"Software", "Sistemas", "Systems", "Web"]

However, you can easily change or improve these terms by editing the `config.toml` file.


### If you are looking for a job

If your looking for a job, you can change these terms by editing the `config.toml` file
and adding, for example, the following terms:

position_filter_for = ["Attraction", "Atração", "Benefit", "Benefits",
"Benefício", "Benefícios", "Gente", "HR", "Human Resources", "People",
"R&S", "Recruiter", "Recrutador", "Recrutadora", "Recursos Humanos",
"RH", "Talent", "Talento", "Talentos", "Headhunter"]
