package ArquivoComHeaderCorpoTrailer;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
//import java.text.SimpleDateFormat;
//import java.util.Date;

public class LeArquivo {
		
	public static void leArquivo(String nomeArq) {
		BufferedReader entrada = null;
		String registro;
		String tipoRegistro;
		String curso, nomeJogador, email, genero;
		double idade;
		int vitorias, contRegistro=0;
		int empates;
		int derrotas;
		int moedas;
		int tirosCertos;
		int tirosRuins;
		
		// Abre o arquivo
		try {
			entrada = new BufferedReader(new FileReader(nomeArq));
		} catch (IOException e) {
			System.err.printf("Erro na abertura do arquivo: %s.\n", e.getMessage());
		}
		
		// L� os registros do arquivo
		try {
			// L� um registro
			registro = entrada.readLine();
			
			while (registro != null) {
				// Obt�m o tipo do registro
				tipoRegistro = registro.substring(0, 2);
				
				if (tipoRegistro.equals("00")) {
					System.out.println("Header");
					System.out.println("Tipo de arquivo: " + registro.substring(2, 6));
					int periodoLetivo= Integer.parseInt(registro.substring(6,11));
					System.out.println("Per�odo letivo: " + periodoLetivo);
					System.out.println("Data/hora de gera��o do arquivo: " + registro.substring(11,30));
					System.out.println("Vers�o do layout: " + registro.substring(30,32));
				}
				else if (tipoRegistro.equals("01")) {
					System.out.println("\nTrailer");
					int qtdRegistro = Integer.parseInt(registro.substring(2,12));
					if (qtdRegistro == contRegistro) {
						System.out.println("Quantidade de registros gravados compat�vel com quantidade lida");
					}	
					else {	
						System.out.println("Quantidade de registros gravados n�o confere com quantidade lida");
					}
				}
				else if (tipoRegistro.contentEquals("02")) {
					if (contRegistro == 0) {
						System.out.println();
						System.out.printf("%-5s %-50s %-40s %-2s %3s %9s %9s %9s %9s %9s %9s\n", "N° PLAYER","NOME DO JOGADOR","EMAIL","GENERO",
								"IDADE", "VITORIAS", "EMPATES", "DERROTAS","MOEDAS", "TIROS CERTOS", "TIROS RUINS");

					}
					
					curso = registro.substring(2,7);
					nomeJogador = registro.substring(7,57);
					email = registro.substring(57,97);
					genero = registro.substring(97,99);
					idade = Double.parseDouble(registro.substring(99,102).replace(',','.'));
					vitorias = Integer.parseInt(registro.substring(102,111));
					empates = Integer.parseInt(registro.substring(111,120));
					derrotas = Integer.parseInt(registro.substring(120,129));
					moedas = Integer.parseInt(registro.substring(129,138));
					tirosCertos = Integer.parseInt(registro.substring(138,147));
					tirosRuins = Integer.parseInt(registro.substring(147,156));
					
					System.out.printf("%-5s %-8s %-50s %-40s %5.2f %6d\n", curso, nomeJogador, email,genero,
							                                               idade, vitorias, empates, derrotas, moedas, tirosCertos, tirosRuins);
					contRegistro++;
				}
				else {
					System.out.println("Tipo de registro inv�lido");
				}
				
				// l� o pr�ximo registro
				registro = entrada.readLine();
			}
			
			// Fecha o arquivo
		    entrada.close();
		} catch (IOException e) {
			System.err.printf("Erro ao ler arquivo: %s.\n", e.getMessage());
		}
			
	}

	public static void main(String[] args) {
		String nomeArq = "ArquivoNotas";
		leArquivo(nomeArq);
	}

}
