#include "mainwindow.h"
#include "./ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    ui->Btn_Open_Conn->setEnabled(true);
    ui->Btn_Close_Conn->setEnabled(false);
    ui->Btn_Open_file->setEnabled(false);
    ui->Btn_Stream->setEnabled(false);
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::Disconnect()
{
    Socket->deleteLater();
    Socket=nullptr;
    WriteMessage("Socket disconnected");
}

void MainWindow::WriteMessage(QString Mssg)
{
    QString tgl = (QDate::currentDate()).toString("yyyy-MM-dd");
    QString wkt = (QTime::currentTime()).toString("hh:mm:ss");
    QStringList mlines = ui->Message_Viewer->toPlainText().split('\n').mid(0, 99);
    mlines.prepend(tgl + ", " + wkt + ", " + Mssg);

    ui->Message_Viewer->setPlainText(mlines.join("\n"));
}

void MainWindow::on_Btn_Open_Conn_clicked()
{
    Socket = new QTcpSocket(this);

    connect(Socket,&QTcpSocket::disconnected,this, &MainWindow::Disconnect);

    Socket->connectToHost(QHostAddress::LocalHost,20508);

    if(Socket->waitForConnected())
        {
        WriteMessage("Connected to BADAQ Processing");
        ui->Btn_Open_Conn->setEnabled(false);
        ui->Btn_Close_Conn->setEnabled(true);
        ui->Btn_Open_file->setEnabled(true);
        ui->Btn_Stream->setEnabled(false);
        }
    else
        WriteMessage("ERROR " + Socket->errorString());

}


void MainWindow::on_Btn_Close_Conn_clicked()
{
    if(Socket->isOpen())
    {
        Socket->close();
        WriteMessage("Connection has been closed");
    }
}


void MainWindow::on_Btn_Stream_clicked()
{
    if(Socket)
    {
        if(Socket->isOpen())
        {
            QDataStream SocketStream(Socket);
            SocketStream.setVersion(QDataStream::Qt_6_0);
            QString DataString;
            DataString="";
            int idStart;

            //CHECK FOR PHASE
            switch(Phase)
            {
            case 0: //..............................................PRESQ
                for(int i=0;i<5;i++)
                {
                    DataString=DataString + List[i] + "\n";
                    idLine=idLine+1;                    
                }
                Phase=1;
                ui->Btn_Stream->setText("Stream CORR1");
                break;
            case 1: //..............................................CORR1
                idStart=idLine;

                for(int i=0;i<109;i++)
                {
                    DataString=DataString + List[i+idStart] + "\n";
                    idLine=idLine+1;
                }

                Phase=3;
                ui->Btn_Stream->setText("Stream DSN");
                break;
            case 2: //..............................................START OR END OF SEQUENCE OF DATA
                DataString=List[idLine] + "\n";
                idLine=idLine+1;
                if(Seq==1)
                {
                    Phase=3;
                    ui->Btn_Stream->setText("Stream DSN");
                }
                else
                {
                    Phase=4;
                    ui->Btn_Stream->setText("Stream CORR2");
                }
                break;
            case 3: //..............................................DATA SEQUENCE
                idStart=idLine;

                for(int i=0;i<108;i++)
                {
                    DataString=DataString + List[i+idStart] + "\n";
                    idLine=idLine+1;
                }

                if(List[idLine].left(3)=="END")//..........................Check end of DATA SEQUENCE
                {
                    if(List[idLine+1].left(3)=="SEQ")
                    {
                        Phase=2; //........................................START of another SEQUENCE OF DATA
                        Seq=1;
                    }
                    else
                    {
                        Phase=2; //........................................Move to CORR2
                        Seq=0;
                    }
                    ui->Btn_Stream->setText("Stream ENDSEQ");

                }
                else Phase=3; //...........................................Next data sequence

                break;
            case 4: //..............................................CORR2
                idStart=idLine;

                for(int i=0;i<108;i++)
                {
                    DataString=DataString + List[i+idStart] + "\n";
                    idLine=idLine+1;
                }

                Phase=5;
                ui->Btn_Stream->setText("Stream ENDRUN");
                break;
            case 5: //..............................................END OF RUN
                DataString=List[idLine];
                idLine=0;
                Phase=0;
                Seq=1;
                ui->Btn_Stream->setText("Stream PRESQ");
                ui->Btn_Stream->setEnabled(false);
                break;
            }

            //STREAMING
            QByteArray byteArray = DataString.toUtf8();
            SocketStream << byteArray;
            WriteMessage("\n" + DataString);
        }
    }
}


void MainWindow::on_Btn_Open_file_clicked()
{
    QFile File(QFileDialog::getOpenFileName(this, tr("Open Raw Data File"),"",tr("Text Files(*.txt)")));
    if(File.open(QIODevice::ReadOnly | QIODevice::Text))
    {
        QTextStream TextStream(&File);

        while(!TextStream.atEnd())
            List+=TextStream.readLine().split("/n");

        Phase=0; //PRESQ
        Length=List.length();
        idLine=0;

        ui->Btn_Open_Conn->setEnabled(false);
        ui->Btn_Close_Conn->setEnabled(true);
        ui->Btn_Open_file->setEnabled(true);
        ui->Btn_Stream->setEnabled(true);

        WriteMessage("Raw data of " + List[0] + " " + List[1] + " is loaded");
    }
    else WriteMessage("ERROR Raw data file can not be opened");

}

